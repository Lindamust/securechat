use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Request as AxumRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use frunk::hlist::HList;
use serde::{Serialize, de::DeserializeOwned};
use validator::Validate;

use crate::{
    error::HttpResult,
    extractors::{
        auth::{AuthIdentity, Identity},
        valid::ValidatedJson,
    },
    traits::{CommandExecutor, InfraCommand, IntoCommand},
};

use pipeline_core::{
    chain::{ExecuteChain, StepChain},
    error::PipelineResult,
    hlist::IntoHList,
    request::Request,
    stages::Executed,
};

// Route pipeline

// ── RunFn — type-erased pipeline body ─────────────────────────────────────────
//
// Arc<dyn Fn> is used so Pipeline can derive Clone cheaply.
// The Arc is cloned once per request (one atomic refcount increment).

type RunFn<I, Exec> = Arc<
    dyn Fn(Identity, I, Arc<Exec>) -> Pin<Box<dyn Future<Output = HttpResult<Response>> + Send>>
        + Send
        + Sync,
>;

// ── Pipeline ──────────────────────────────────────────────────────────────────

pub struct Pipeline<I, O, Exec> {
    require_auth: bool,
    run_fn: RunFn<I, Exec>,
    _phantom: PhantomData<fn() -> O>, // fn() -> O is Send+Sync regardless of O
}

// Clone is cheap, but impl manually because Exec is ?Sized
impl<I, O, Exec> Clone for Pipeline<I, O, Exec> {
    fn clone(&self) -> Self {
        Self {
            require_auth: self.require_auth,
            run_fn: self.run_fn.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<I, O, Exec> Pipeline<I, O, Exec>
where
    Exec: Send + Sync + 'static,
{
    // ── IntoCommand path ──────────────────────────────────────────────────────

    pub fn new(
        require_auth: bool,
        map_resp: fn(Request<Executed, <I::Command as InfraCommand>::Output>) -> PipelineResult<O>,
    ) -> Self
    where
        I: IntoCommand + Send + 'static,
        I::Command: Send + 'static,
        <I::Command as InfraCommand>::Output: Send + 'static,
        O: Serialize + Send + Sync + 'static,
        Exec: CommandExecutor<I::Command>,
    {
        let run_fn = Arc::new(move |identity: Identity, input: I, executor: Arc<Exec>| {
            Box::pin(async move {
                let cmd = input.into_command(&identity);
                let executed = executor.execute(cmd).await?;
                let body = map_resp(executed)?;
                Ok((StatusCode::CREATED, Json(body)).into_response())
            }) as Pin<Box<dyn Future<Output = HttpResult<Response>> + Send>>
        });

        Self {
            require_auth,
            run_fn,
            _phantom: PhantomData,
        }
    }

    // ── StepChain path ────────────────────────────────────────────────────────
    //
    // Steps are ZST post compile -> Clone is free.
    // The chain is cloned once per request
    // (inside the Fn closure) at zero runtime cost.

    pub fn new_chained<Steps, H, ChainOut, Idx>(
        require_auth: bool,
        chain: StepChain<Steps, Exec>,
        map_resp: fn(Request<Executed, ChainOut>) -> PipelineResult<O>,
    ) -> Self
    where
        I: IntoHList<Output = H> + Send + 'static,
        H: HList + Send + 'static,
        Steps: ExecuteChain<frunk::HCons<Identity, H>, Exec, Output = ChainOut>
            + Clone
            + Send
            + Sync
            + 'static,
        ChainOut: Send + 'static,
        O: Serialize + Send + Sync + 'static,
    {
        let run_fn = Arc::new(move |identity: Identity, input: I, executor: Arc<Exec>| {
            let chain = chain.clone();
            Box::pin(async move {
                let hlist = frunk::HCons {
                    head: identity,
                    tail: input.into_hlist(),
                };
                let output = chain.run(hlist, &*executor).await?;
                let body = map_resp(Request::wrap(output))?;
                Ok((StatusCode::CREATED, Json(body)).into_response())
            }) as Pin<Box<dyn Future<Output = HttpResult<Response>> + Send>>
        });

        Self {
            require_auth,
            run_fn,
            _phantom: PhantomData,
        }
    }

    // ── Shared run ────────────────────────────────────────────────────────────

    async fn run(self, identity: Identity, input: I, executor: Arc<Exec>) -> Response {
        let result = (self.run_fn)(identity, input, executor).await;
        result.unwrap_or_else(IntoResponse::into_response)
    }
}

// ── Axum Handler impl ─────────────────────────────────────────────────────────

impl<I, O, Exec> axum::handler::Handler<((),), Arc<Exec>> for Pipeline<I, O, Exec>
where
    I: DeserializeOwned + Validate + Send + 'static,
    O: Send + Sync + 'static, // Sync required: PhantomData<O> in Pipeline
    Exec: Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: AxumRequest, state: Arc<Exec>) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            // Auth extraction — before body is consumed.
            let identity = match AuthIdentity::from_request_parts(&mut parts, &()).await {
                Ok(AuthIdentity(id)) => id,
                Err(e) => return e.into_response(),
            };

            // Fail fast on auth before parsing body.
            if self.require_auth {
                if let Err(e) = identity.require_authenticated() {
                    return e.into_response();
                }
            }

            let req_for_json = AxumRequest::from_parts(parts, body);
            let ValidatedJson(input) =
                match ValidatedJson::<I>::from_request(req_for_json, &()).await {
                    Ok(v) => v,
                    Err(e) => return e,
                };

            self.run(identity, input, state).await
        })
    }
}

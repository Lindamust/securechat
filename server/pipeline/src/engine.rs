use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Request as AxumRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use validator::Validate;

use crate::{
    extractors::{
        auth::{AuthIdentity, Identity, into_authenticated},
        valid::ValidatedJson,
    },
    traits::{CommandExecutor, InfraCommand, IntoCommand},
    typestate::{error::PipelineResult, request::Request, stages::Executed},
};

// Route pipeline

pub struct Pipeline<I, O>
where
    I: IntoCommand,
{
    pub require_auth: bool,
    pub map_resp: fn(Request<Executed, <I::Command as InfraCommand>::Output>) -> PipelineResult<O>,

    _phantom: PhantomData<(I, O)>,
}

impl<I, O> Pipeline<I, O>
where
    I: IntoCommand,
{
    pub fn new(
        require_auth: bool,
        map_resp: fn(Request<Executed, <I::Command as InfraCommand>::Output>) -> PipelineResult<O>,
    ) -> Self {
        Self {
            require_auth,
            map_resp,
            _phantom: PhantomData,
        }
    }

    pub async fn run<Exec>(self, identity: Identity, input: I, executor: &Exec) -> Response
    where
        Exec: CommandExecutor<I::Command>,
        O: Serialize,
    {
        let result: PipelineResult<Response> = async {
            // Auth gate.
            let auth_req = into_authenticated(input, identity, self.require_auth)?;
            let (identity, input) = auth_req.into_inner();

            // IntoCommand — zero cost for trivial case, explicit work for non-trivial.
            let cmd = input.into_command(&identity);

            // The one impure step.
            let executed_req = executor.execute(cmd).await?;

            // Map response.
            let body = (self.map_resp)(executed_req)?;
            Ok((StatusCode::CREATED, Json(body)).into_response())
        }
        .await;

        result.unwrap_or_else(IntoResponse::into_response)
    }
}

impl<I, O> Copy for Pipeline<I, O> where I: IntoCommand {}

impl<I, O> Clone for Pipeline<I, O>
where
    I: IntoCommand,
{
    fn clone(&self) -> Self {
        *self
    }
}
// Json Validation Extractor

// Pipeline Axum Integration

impl<I, O, Exec> axum::handler::Handler<((),), Arc<Exec>> for Pipeline<I, O>
where
    I: IntoCommand + DeserializeOwned + Validate + Send + 'static + Sync,
    I::Command: InfraCommand + Send + 'static + Sync,
    <I::Command as InfraCommand>::Output: Send + 'static + Sync,
    O: Serialize + Send + 'static + Sync,
    Exec: CommandExecutor<I::Command> + Clone + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: AxumRequest, state: Arc<Exec>) -> Self::Future {
        Box::pin(async move {
            // Split parts so we can run two extractors on the same request.
            let (mut parts, body) = req.into_parts();

            // 1. Extract identity from headers
            let identity = match AuthIdentity::from_request_parts(&mut parts, &()).await {
                Ok(AuthIdentity(id)) => id,
                Err(e) => return e.into_response(),
            };

            // 2. Extract + validate body via ValidatedJson<I>
            //    nutype Deserialize runs: validator::Validate runs.
            //    On failure: 422 with a structured error body.
            let reassembled = axum::extract::Request::from_parts(parts, body);
            let ValidatedJson(input) =
                match ValidatedJson::<I>::from_request(reassembled, &()).await {
                    Ok(v) => v,
                    Err(e) => return e.into_response(),
                };

            self.run(identity, input, &*state).await
        })
    }
}

use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use axum::{
    extract::{FromRequest, FromRequestParts, Request as AxumRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,

};
use axum_valid::Valid;
use serde::{de::DeserializeOwned, Serialize};
use validator::Validate;

use super::{
    auth::{into_authenticated, AuthIdentity, Identity},
    error::PipelineResult,
    request::Request,
    stages::{CommandReady, Executed, Validated},
};
use crate::{Command, CommandExecutor};


pub struct Pipeline<I, C, O>
where
    C: Command,
{
    pub require_auth: bool,
    pub build_cmd: fn(Request<Validated, I>, &Identity) -> PipelineResult<Request<CommandReady, C>>,
    pub map_resp: fn(Request<Executed, C::Output>) -> PipelineResult<O>,

    _phantom: PhantomData<(I, C, O)>,
}

impl<I, C, O> Copy for Pipeline<I, C, O> where C: Command {}

impl<I, C, O> Clone for Pipeline<I, C, O>
where
    C: Command,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<I, C, O> Pipeline<I, C, O>
where
    C: Command,
{
    pub fn new(
        require_auth: bool,
        build_cmd: fn(Request<Validated, I>, &Identity) -> PipelineResult<Request<CommandReady, C>>,
        map_resp: fn(Request<Executed, C::Output>) -> PipelineResult<O>,
    ) -> Self {
        Self {
            require_auth,
            build_cmd,
            map_resp,
            _phantom: PhantomData,
        }
    }
}

impl<I, C, O> Pipeline<I, C, O>
where
    I: Send + 'static,
    C: Command + Send + 'static,
    C::Output: Send + 'static,
    O: Serialize + Send + 'static,
{
    pub async fn run<Exec>(self, identity: Identity, input: I, executor: &Exec) -> Response
    where 
        Exec: CommandExecutor<C> + Clone + Send + Sync + 'static,
    {
        let result: PipelineResult<Response> = async {
            let auth_req = into_authenticated(input, identity, self.require_auth)?;
            let (identity, input) = auth_req.into_inner();

            let validated_req: Request<Validated, I> = Request::new(input);

            let cmd_req = (self.build_cmd)(validated_req, &identity)?;

            let executed_req = executor.execute(cmd_req).await?;

            let body: O = (self.map_resp)(executed_req)?;

            // TODO: sometimes the success return could be other than 200 OK
            Ok((StatusCode::OK, Json(body)).into_response())
        }
        .await;

        result.unwrap_or_else(IntoResponse::into_response)
    }
}

impl<I, C, O, Exec> axum::handler::Handler<((),), Arc<Exec>> for Pipeline<I, C, O>
where
    I: DeserializeOwned + Validate + Send + 'static + Sync,
    C: Command + Send + 'static + Sync,
    C::Output: Send + 'static + Sync,
    O: Serialize + Send + 'static + Sync,
    Exec: CommandExecutor<C> + Clone + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: AxumRequest, state: Arc<Exec>) -> Self::Future {
        Box::pin(async move {
            // Split parts so we can run two extractors on the same request.
            let (mut parts, body) = req.into_parts();

            // 1. Extract identity from headers (never consumes body).
            let identity = match AuthIdentity::from_request_parts(&mut parts, &()).await {
                Ok(AuthIdentity(id)) => id,
                Err(e) => return e.into_response(),
            };

            // 2. Extract + validate body via Valid<Json<I>>.
            //    nutype Deserialize runs → validator::Validate runs.
            //    Any failure → 422 with a structured error body.
            let reassembled = axum::extract::Request::from_parts(parts, body);
            let Valid(Json(input)) =
                match Valid::<Json<I>>::from_request(reassembled, &state).await {
                    Ok(v) => v,
                    Err(e) => return e.into_response(),
                };

            self.run(identity, input, &*state).await
        })
    }
}

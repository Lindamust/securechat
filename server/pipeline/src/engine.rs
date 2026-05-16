use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc, usize};

use axum::{
    Json,
    extract::{FromRequestParts, Request as AxumRequest},
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    Command, CommandExecutor,
    auth::{AuthIdentity, Identity, into_authenticated},
    error::PipelineResult,
    request::Request,
    stages::{CommandReady, Dto, Executed, Validated},
};

pub struct Pipeline<A, B, C, D>
where
    C: Command,
{
    pub require_auth: bool,
    pub validate: fn(Request<Dto, A>) -> PipelineResult<Request<Validated, B>>,
    pub build_cmd: fn(Request<Validated, B>, &Identity) -> PipelineResult<Request<CommandReady, C>>,
    pub map_resp: fn(Request<Executed, C::Output>) -> PipelineResult<D>,

    _phantom: PhantomData<(A, B, C, D)>,
}

impl<A, B, C, D> Copy for Pipeline<A, B, C, D> where C: Command {}

impl<A, B, C, D> Clone for Pipeline<A, B, C, D>
where
    C: Command,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<A, B, C, D> Pipeline<A, B, C, D>
where
    C: Command,
{
    pub fn new(
        require_auth: bool,
        validate: fn(Request<Dto, A>) -> PipelineResult<Request<Validated, B>>,
        build_cmd: fn(Request<Validated, B>, &Identity) -> PipelineResult<Request<CommandReady, C>>,
        map_resp: fn(Request<Executed, C::Output>) -> PipelineResult<D>,
    ) -> Self {
        Self {
            require_auth,
            validate,
            build_cmd,
            map_resp,
            _phantom: PhantomData,
        }
    }
}

impl<A, B, C, D> Pipeline<A, B, C, D>
where
    A: DeserializeOwned + Send + 'static,
    B: Send + 'static,
    C: Command + Send + 'static,
    D: Serialize + Send + 'static,
{
    pub async fn run<Exec>(self, identity: Identity, dto: A, executor: &Exec) -> Response
    where
        Exec: CommandExecutor<C> + Clone + Send + Sync + 'static,
    {
        let result: PipelineResult<Response> = async {
            let auth_req = into_authenticated(dto, identity, self.require_auth)?;
            let (identity, dto) = auth_req.into_inner();

            let dto_req: Request<Dto, A> = Request::new(dto);

            let validated_req = (self.validate)(dto_req)?;

            let cmd_req = (self.build_cmd)(validated_req, &identity)?;

            let executed_req = executor.execute(cmd_req).await?;

            let body = (self.map_resp)(executed_req)?;

            Ok((StatusCode::OK, Json(body)).into_response())
        }
        .await;

        result.unwrap_or_else(IntoResponse::into_response)
    }
}

impl<A, B, C, D, Exec> Handler<((),), Arc<Exec>> for Pipeline<A, B, C, D>
where
    A: DeserializeOwned + Send + Sync + 'static,
    B: Send + Sync + 'static,
    C: Command + Send + Sync + 'static,
    C::Output: Send + Sync + 'static,
    D: Serialize + Send + Sync + 'static,
    Exec: CommandExecutor<C> + Clone + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: AxumRequest, state: Arc<Exec>) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let identity = match AuthIdentity::from_request_parts(&mut parts, &()).await {
                Ok(AuthIdentity(id)) => id,
                Err(e) => return e.into_response(),
            };

            let bytes = match axum::body::to_bytes(body, usize::MAX).await {
                Ok(b) => b,
                Err(e) => return e.to_string().into_response(),
            };

            let dto: A = match serde_json::from_slice(&bytes) {
                Ok(v) => v,
                Err(e) => {
                    return (StatusCode::BAD_REQUEST, format!("Invalid JSON: {e}")).into_response();
                }
            };

            self.run(identity, dto, &*state).await
        })
    }
}

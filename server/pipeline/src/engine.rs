use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Request as AxumRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use validator::Validate;

use super::{
    auth::{AuthIdentity, Identity, into_authenticated},
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

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
    <Json<T> as FromRequest<S>>::Rejection: IntoResponse,
{
    type Rejection = Response;

    async fn from_request(req: AxumRequest, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        value.validate().map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": e.to_owned() })),
            )
                .into_response()
        })?;

        Ok(ValidatedJson(value))
    }
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

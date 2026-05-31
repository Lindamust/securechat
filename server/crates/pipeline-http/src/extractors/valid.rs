use axum::{
    Json,
    extract::{FromRequest, Request as AxumRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

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

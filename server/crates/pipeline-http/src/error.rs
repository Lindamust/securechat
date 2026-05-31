use axum::{Json, http::StatusCode, response::IntoResponse};
use pipeline_core::error::PipelineError;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

#[derive(Debug)]
pub struct HttpError(pub PipelineError);

pub type HttpResult<T> = Result<T, HttpError>;

impl From<PipelineError> for HttpError {
    fn from(value: PipelineError) -> Self {
        Self(value)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.0 {
            PipelineError::MissingToken | PipelineError::InvalidToken => StatusCode::UNAUTHORIZED,
            PipelineError::Forbidden => StatusCode::FORBIDDEN,
            PipelineError::Validation(_)
            | PipelineError::WeakPassword(_, _)
            | PipelineError::ParseError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            PipelineError::Conflict(_) => StatusCode::CONFLICT,
            PipelineError::NotFound(_) => StatusCode::NOT_FOUND,
            PipelineError::Database(_) | PipelineError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let message = match &self.0 {
            PipelineError::Database(_) | PipelineError::Internal(_) => {
                "an unexpected error occurred".to_owned()
            }
            other => other.to_string(),
        };

        (status, Json(ErrorBody { error: message })).into_response()
    }
}

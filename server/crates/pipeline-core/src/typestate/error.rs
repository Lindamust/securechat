use axum_error_handler::AxumErrorResponse;
use thiserror::Error;

pub type PipelineResult<T> = Result<T, PipelineError>;

#[derive(Debug, Error, AxumErrorResponse)]
pub enum PipelineError {
    #[status_code("401")]
    #[code("UNAUTHORISED")]
    #[error("missing or malformed Authorization header")]
    MissingToken,

    #[status_code("401")]
    #[code("UNAUTHORISED")]
    #[error("JWT is invalid or expired")]
    InvalidToken,

    #[status_code("403")]
    #[code("FORBIDDEN")]
    #[error("insufficient permissions")]
    Forbidden,

    #[status_code("422")]
    #[code("UNPROCESSABLE_ENTITY")]
    #[error("validation failed: {0}")]
    Validation(String),

    #[status_code("422")]
    #[code("UNPROCESSABLE_ENTITY")]
    #[error("password too weak (score {0}/4): {1}")]
    WeakPassword(u8, String),

    #[status_code("422")]
    #[code("UNPROCESSABLE_ENTITY")]
    #[error("parse error: {0}")]
    ParseError(#[from] serde::de::value::Error),

    #[status_code("409")]
    #[code("CONFLICT")]
    #[error("conflict: {0}")]
    Conflict(String),

    #[status_code("404")]
    #[code("NOT_FOUND")]
    #[error("not found: {0}")]
    NotFound(String),

    #[status_code("500")]
    #[code("INTERNAL_SERVER_ERROR")]
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[status_code("500")]
    #[code("INTERNAL_SERVER_ERROR")]
    #[error("internal error: {0}")]
    Internal(String),
}

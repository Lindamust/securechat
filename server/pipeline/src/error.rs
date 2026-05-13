use axum_error_handler::AxumErrorResponse;
use thiserror::Error;

pub type PipelineResult<T> = Result<T, PipelineError>;

#[derive(Debug, Error, AxumErrorResponse)]
pub enum PipelineError {
    // Auth
    #[code("401")]
    #[status_code("UNAUTHORISED")]
    #[error("missing or malformed Authorization header")]
    MissingToken,

    #[code("401")]
    #[status_code("UNAUTHORISED")]
    #[error("JWT is invalid or expired")]
    InvalidToken,

    #[code("403")]
    #[status_code("FORBIDDEN")]
    #[error("insufficient permissions")]
    Forbidden,

    // Validation
    #[code("422")]
    #[status_code("UNPROCESSABLE_ENTITY")]
    #[error("validation failed: {0}")]
    Validation(String),

    #[code("422")]
    #[status_code("UNPROCESSABLE_ENTITY")]
    #[error("password too weak (score {0}/4): {1}")]
    WeakPassword(u8, String),

    // Command / Domain
    #[code("409")]
    #[status_code("CONFLICT")]
    #[error("conflict: {0}")]
    Conflict(String),

    #[code("404")]
    #[status_code("NOT_FOUND")]
    #[error("not found: {0}")]
    NotFound(String),

    // Infrastructure
    #[code("500")]
    #[status_code("INTERNAL_SERVER_ERROR")]
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[code("500")]
    #[status_code("INTERNAL_SERVER_ERROR")]
    #[error("internal error: {0}")]
    Internal(String),
}

pub type PipelineResult<T> = Result<T, PipelineError>;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("missing or malformed Authorization header")]
    MissingToken,

    #[error("JWT is invalid or expired")]
    InvalidToken,

    #[error("insufficient permissions")]
    Forbidden,

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("password too weak (score {0}/4): {1}")]
    WeakPassword(u8, String),

    #[error("parse error: {0}")]
    ParseError(#[from] serde::de::value::Error),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("internal error: {0}")]
    Internal(String),
}

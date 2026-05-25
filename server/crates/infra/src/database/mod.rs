use pipeline_core::error::PipelineError;
use sqlx::PgPool;

mod insert_auth_challenge;
mod insert_user;

/// Postgres executor
#[derive(Clone)]
pub struct PgDatabase {
    pub pool: PgPool,
}

fn db_err(e: sqlx::Error) -> PipelineError {
    PipelineError::Database(e.to_string())
}

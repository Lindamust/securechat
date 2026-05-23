use sqlx::PgPool;

mod insert_auth_challenge;
mod insert_user;

/// Postgres executor
#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

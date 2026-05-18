use commands::{AuthChallengeNonce, CreateAuthCommand};
use pipeline::{CommandExecutor, error::PipelineError, primitives::{Bytes32, Nonce}, request::Request};
use sqlx::types::chrono::Utc;

use crate::database::PgExecutor;

impl CommandExecutor<CreateAuthCommand> for PgExecutor {
    async fn execute(
        &self,
        req: pipeline::request::Request<pipeline::stages::CommandReady, CreateAuthCommand>,
    ) -> Result<pipeline::request::Request<pipeline::stages::Executed, <CreateAuthCommand as pipeline::Command>::Output>, pipeline::error::PipelineError>
    {
        let cmd = req.into_inner();
        
        let nonce = Nonce::generate();

        let raw_bytes = sqlx::query_scalar!(
            r#"
            INSERT INTO auth_challeneges (nonce, user_id, expires_at)
            VALUES ($1, (SELECT id FROM users WHERE ik_pub = $2), $3)
            RETURNING nonce
            "#,
            nonce.0.0 as _,
            cmd.ik_pub.0.0 as _,
            Utc::now(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(PipelineError::Database)?;

        let nonce = Bytes32::try_from(raw_bytes)
            .map_err(|_| PipelineError::Internal("Couldn't parse raw bytes returned from database".to_owned()))
            .map(Nonce)?;

        Ok(Request::new(AuthChallengeNonce {
            nonce,
        }))


    }
}
use chrono::{Duration, Utc};
use pipeline::commands::{AuthChallengeNonce, CreateAuthCommand};
use pipeline::{
    CommandExecutor,
    error::{PipelineError, PipelineResult},
    primitives::Nonce,
    request::Request,
    stages::{CommandReady, Executed},
};

use crate::database::PgExecutor;

impl CommandExecutor<CreateAuthCommand> for PgExecutor {
    async fn execute(
        &self,
        req: Request<CommandReady, CreateAuthCommand>,
    ) -> PipelineResult<Request<Executed, AuthChallengeNonce>> {
        let nonce = Nonce::generate();
        let ik_pub = req.into_inner().ik_pub;
        let ttl = Utc::now() + Duration::seconds(30);

        let nonce = sqlx::query_scalar!(
            r#"
            INSERT INTO auth_challenges (nonce, user_id, expires_at)
            SELECT $1, users.id, $3
            FROM users
            WHERE users.ik_pub = $2
            RETURNING nonce as "nonce: Nonce"
            "#,
            nonce as _,
            ik_pub as _,
            ttl,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(PipelineError::Database)?;

        Ok(Request::new(AuthChallengeNonce { nonce }))
    }
}

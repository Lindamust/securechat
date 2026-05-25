use chrono::{Duration, Utc};
use pipeline_core::{
    dto::{AuthChallengeBody, AuthChallengeNonce},
    primitives::Nonce,
    traits::CommandExecutor,
    typestate::{
        error::{PipelineError, PipelineResult},
        request::Request,
        stages::Executed,
    },
};

use crate::database::PgDatabase;

impl CommandExecutor<AuthChallengeBody> for PgDatabase {
    fn execute(
        &self,
        cmd: AuthChallengeBody,
    ) -> impl Future<Output = PipelineResult<Request<Executed, AuthChallengeNonce>>> + Send {
        async move {
            let nonce = Nonce::generate();
            let ik_pub = cmd.ik_pub;
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
}

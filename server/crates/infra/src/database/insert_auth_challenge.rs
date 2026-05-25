use chrono::{Duration, Utc};
use domain::dto::{AuthChallengeBody, InsertedNonce};
use domain::models::Nonce;
use pipeline_core::{request::Request, stages::Executed};
use pipeline_http::error::HttpResult;
use pipeline_http::traits::CommandExecutor;

use crate::database::{PgDatabase, db_err};

impl CommandExecutor<AuthChallengeBody> for PgDatabase {
    fn execute(
        &self,
        cmd: AuthChallengeBody,
    ) -> impl Future<Output = HttpResult<Request<Executed, InsertedNonce>>> + Send {
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
            .map_err(db_err)?;

            Ok(Request::wrap(InsertedNonce { nonce }))
        }
    }
}

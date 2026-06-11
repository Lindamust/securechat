use chrono::{DateTime, Duration, Utc};
use domain::dto::{AuthChallengeBody, InsertedNonce};
use domain::models::{IkPub, NonceKey, NonceType};
use pipeline_core::error::PipelineResult;
use pipeline_core::{request::Request, stages::Executed};
use pipeline_http::error::HttpResult;
use pipeline_http::traits::CommandExecutor;

use uuid::Uuid;

use crate::database::{PgDatabase, db_err};

impl CommandExecutor<AuthChallengeBody> for PgDatabase {
    fn execute(
        &self,
        cmd: AuthChallengeBody,
    ) -> impl Future<Output = HttpResult<Request<Executed, InsertedNonce>>> + Send {
        async move {
            let nonce = NonceKey::generate();
            let ik_pub = cmd.ik_pub;
            let ttl = Utc::now() + Duration::seconds(30);

            // TODO: maybe make user_id unique or Pk
            // then force an update on the nonce if
            // another request is made
            let nonce = sqlx::query_scalar!(
                r#"
                        INSERT INTO auth_challenges (nonce, user_id, expires_at)
                        SELECT $1, users.id, $3
                        FROM users
                        WHERE users.ik_pub = $2
                        RETURNING nonce as "nonce: NonceKey"
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

pub struct NonceRow {
    pub nonce: NonceKey,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

impl PgDatabase {
    pub async fn store_nonce(&self, ik_pub: &IkPub, n: &NonceType) -> PipelineResult<NonceKey> {
        sqlx::query_scalar!(
            r#"
                INSERT INTO auth_challenges (nonce, user_id, expires_at)
                SELECT $1, users.id, $3
                FROM users
                WHERE users.ik_pub = $2
                RETURNING nonce as "nonce: NonceKey"
            "#,
            n.nonce as _,
            ik_pub as _,
            n.expires_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)
    }

    pub async fn get_nonce(&self, ik_pub: &IkPub) -> PipelineResult<NonceRow> {
        sqlx::query_as!(
            NonceRow,
            r#"
                SELECT nonce, user_id, expires_at
                FROM auth_challenges
                INNER JOIN users ON id = user_id
                WHERE id = $1
                AND expires_at >= NOW() - INTERVAL '30 seconds'
            "#,
            ik_pub as _,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)
    }
}

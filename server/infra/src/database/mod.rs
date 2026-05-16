use sqlx::{PgPool, types::chrono::Utc};

use commands::{CreatedUser, RegisterUserCommand, SendMessageCommand, SentMessage};

use pipeline::{
    CommandExecutor,
    error::{PipelineError, PipelineResult},
    request::Request,
    stages::{CommandReady, Executed},
};
use uuid::Uuid;

use crate::crypto::hash_password;

/// Postgres executor
#[derive(Clone)]
pub struct PgExecutor {
    pub pool: PgPool,
}

/// RegisterUser
impl CommandExecutor<RegisterUserCommand> for PgExecutor {
    async fn execute(
        &self,
        req: Request<CommandReady, RegisterUserCommand>,
    ) -> PipelineResult<Request<Executed, CreatedUser>> {
        let cmd = req.into_inner();

        // Hash password — use argon2 / bcrypt in production.
        let password_hash = hash_password(cmd.password.as_str())?;

        let row = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, ik_pub, ik_pub_ed, spk_pub, spk_pub_sig, spk_uploaded_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, username
            "#,
            cmd.username.as_str(),
            cmd.email.as_str(),
            password_hash,
            cmd.ik_pub.0.0 as _,
            cmd.ik_pub_ed.0.0 as _,
            cmd.spk_pub.0.0 as _,
            cmd.spk_pub_sig.0.0 as _,
            Utc::now(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                PipelineError::Conflict(format!(
                    "username '{}' is already taken",
                    cmd.username.as_str()
                ))
            },
            _ => PipelineError::Database(e),
        })?;

        Ok(Request::new(CreatedUser {
            id: row.id,
            username: row.username,
        }))
    }
}

/// SendMessage
impl CommandExecutor<SendMessageCommand> for PgExecutor {
    async fn execute(
        &self,
        req: Request<CommandReady, SendMessageCommand>,
    ) -> PipelineResult<Request<Executed, SentMessage>> {
        let _cmd = req.into_inner();

        Ok(Request::new(SentMessage {
            id: Uuid::now_v7(),
            created_at: Utc::now(),
        }))
    }
}

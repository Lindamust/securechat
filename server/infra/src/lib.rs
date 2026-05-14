use sqlx::{PgPool, types::chrono::{DateTime, Utc}};

use commands::{CreatedUser, RegisterUserCommand, SendMessageCommand, SentMessage};

use pipeline::{
    CommandExecutor,
    error::{PipelineError, PipelineResult},
    request::Request,
    stages::{CommandReady, Executed},
};
use uuid::Uuid;

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
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username
            "#,
            Uuid::now_v7(),
            cmd.username.as_str(),
            cmd.email.as_str(),
            password_hash,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                PipelineError::Conflict(format!(
                    "username '{}' is already taken",
                    cmd.username.as_str()
                ))
            }
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_email_key") => {
                PipelineError::Conflict("that email address is already registered".to_owned())
            }
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
        let cmd = req.into_inner();

        Ok(Request::new(SentMessage {
            id: Uuid::now_v7(),
            created_at: Utc::now(),
        }))
    }
}

// Password hashing (TODO)
fn hash_password(plain: &str) -> Result<String, PipelineError> {
    // Replace with argon2::hash_encoded or bcrypt::hash later
    Ok(format!("hashed:{plain}"))
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

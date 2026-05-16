use sqlx::types::chrono::Utc;

use commands::{CreatedUser, RegisterUserCommand};

use pipeline::{
    CommandExecutor,
    error::{PipelineError, PipelineResult},
    primitives::Bytes32,
    request::Request,
    stages::{CommandReady, Executed},
};

use super::PgExecutor;
use crate::crypto::hash_password;

impl CommandExecutor<RegisterUserCommand> for PgExecutor {
    async fn execute(
        &self,
        req: Request<CommandReady, RegisterUserCommand>,
    ) -> PipelineResult<Request<Executed, CreatedUser>> {
        let cmd = req.into_inner();
        let mut tx = self.pool.begin().await?;

        // Hash password — use argon2 / bcrypt in production.
        let password_hash = hash_password(cmd.password.as_str())?;

        let user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (username, email, password_hash, ik_pub, ik_pub_ed, spk_pub, spk_pub_sig, spk_uploaded_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
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
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                PipelineError::Conflict(format!(
                    "username '{}' is already taken",
                    cmd.username.as_str()
                ))
            },
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_ik_pub_key") => {
                PipelineError::Conflict("duplicate identity key".to_owned())
            },
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_ik_pub_ed_key") => {
                PipelineError::Conflict("duplicate identity key signature".to_owned())
            }
            _ => PipelineError::Database(e),
        })?;

        let otpk_vec_bytes: Vec<Bytes32> = cmd.otkps.iter().map(|otpk_pub| otpk_pub.0).collect();

        sqlx::query!(
            r#"
            INSERT INTO otpks (user_id, otpk_pub)
            SELECT $1, x FROM UNNEST($2::bytea[]) as x
            ON CONFLICT DO NOTHING
            "#,
            user_id,
            otpk_vec_bytes as _,
        )
        .execute(&mut *tx)
        .await?;

        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM otpks
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Request::new(CreatedUser {
            id: user_id,
            otpk_count: count,
        }))
    }
}

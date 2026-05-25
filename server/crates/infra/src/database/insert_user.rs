use crate::database::db_err;

use super::PgDatabase;

use domain::dto::{CreatedUser, RegisterUserCommand};
use pipeline_core::{
    error::{PipelineError, PipelineResult},
    request::Request,
    stages::Executed,
};
use pipeline_http::{error::HttpResult, traits::CommandExecutor};

use chrono::Utc;

impl CommandExecutor<RegisterUserCommand> for PgDatabase {
    fn execute(
        &self,
        cmd: RegisterUserCommand,
    ) -> impl Future<Output = HttpResult<Request<Executed, CreatedUser>>> + Send {
        async move {
            let mut tx = self.pool.begin().await.map_err(db_err)?;

            let id = sqlx::query_scalar!(
                        r#"
                        INSERT INTO users (username, email, password_hash, ik_pub, ik_pub_ed, spk_pub, spk_pub_sig, spk_uploaded_at)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                        RETURNING id
                        "#,
                        cmd.username.as_str(),
                        cmd.email.as_str(),
                        cmd.hashed_password.as_str(),
                        cmd.ik_pub as _,
                        cmd.ik_pub_ed as _,
                        cmd.spk_pub as _,
                        cmd.spk_pub_sig as _,
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
                        _ => db_err(e),
                    })?;

            let inserted = sqlx::query!(
                r#"
                        INSERT INTO otpks (user_id, otpk_pub)
                        SELECT $1, x FROM UNNEST($2::bytea[]) as x
                        ON CONFLICT DO NOTHING
                        "#,
                id,
                cmd.otpks as _,
            )
            .execute(&mut *tx)
            .await
            .map_err(db_err)?
            .rows_affected() as i64;

            tx.commit().await.map_err(db_err)?;

            Ok(Request::wrap(CreatedUser { id, inserted }))
        }
    }
}

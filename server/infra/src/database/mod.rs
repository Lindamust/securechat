use sqlx::{PgPool, types::chrono::Utc};

use commands::{SendMessageCommand, SentMessage};

use pipeline::{
    CommandExecutor,
    error::PipelineResult,
    request::Request,
    stages::{CommandReady, Executed},
};
use uuid::Uuid;

mod insert_user;

/// Postgres executor
#[derive(Clone)]
pub struct PgExecutor {
    pub pool: PgPool,
}

/// SendMessage TODO: Remove
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

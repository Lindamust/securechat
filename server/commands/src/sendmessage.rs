use uuid::Uuid;

use domain::dto::SendMessageInput;
use domain::models::MessageContent;
use pipeline::{
    Command,
    request::Request,
    stages::{CommandReady, Validated},
};

#[derive(Debug)]
pub struct SendMessageCommand {
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: MessageContent,
}

#[derive(Debug)]
pub struct SentMessage {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Command for SendMessageCommand {
    type Output = SentMessage;
}

pub fn build_send_message_command(
    req: Request<Validated, SendMessageInput>,
    sender_id: Uuid,
) -> Request<CommandReady, SendMessageCommand> {
    let input = req.into_inner();
    Request::new(SendMessageCommand {
        sender_id,
        recipient_id: input.recipient_id,
        content: input.content,
    })
}

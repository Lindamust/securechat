use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::MessageContent;

use crate::{
    error::PipelineResult,
    request::Request,
    stages::{Dto, Validated},
};

/// Raw inbound JSON for POST /messages
#[derive(Debug, Deserialize)]
pub struct SendMessageDto {
    pub recipient_id: Uuid,
    pub content: String,
}

/// Validated counterpart.
#[derive(Debug)]
pub struct SendMessageInput {
    pub recipient_id: Uuid, // UUIDs need no further domain validation
    pub content: MessageContent,
}

pub fn validate_send_message(
    req: Request<Dto, SendMessageDto>,
) -> PipelineResult<Request<Validated, SendMessageInput>> {
    let dto = req.into_inner();
    let validated = SendMessageInput {
        recipient_id: dto.recipient_id,
        content: MessageContent::parse(dto.content)?,
    };
    Ok(Request::new(validated))
}

#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub message_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

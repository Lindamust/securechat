mod register;
mod sendmessage;

pub use register::{RegisterDto, RegisterInput, RegisterResponse, validate_register};
pub use sendmessage::{
    SendMessageDto, SendMessageInput, SendMessageResponse, validate_send_message,
};

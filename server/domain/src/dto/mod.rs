mod register;
mod sendmessage;

pub use register::{RegisterDto, RegisterInput, RegisterResponse, validate_register};
pub use sendmessage::{
    SendMessageDto, SendMessageInput, SendMessageResponse, validate_send_message,
};

use serde::de::DeserializeOwned;
use serde::de::IntoDeserializer;
use serde::de::value::{Error as DeserializeError, StringDeserializer};

fn decode<T: DeserializeOwned>(s: String) -> Result<T, DeserializeError> {
    T::deserialize::<StringDeserializer<DeserializeError>>(s.into_deserializer())
}

mod auth_challenge;
mod register;
mod sendmessage;
mod signed_token;

use crate::error::PipelineResult;
use crate::request::Request;
use crate::stages::Dto;
use crate::stages::Validated;
pub use auth_challenge::*;
pub use register::*;
pub use sendmessage::*;

use serde::de::DeserializeOwned;
use serde::de::IntoDeserializer;
use serde::de::value::{Error as DeserializeError, StringDeserializer};

fn decode<T: DeserializeOwned>(s: String) -> Result<T, DeserializeError> {
    T::deserialize::<StringDeserializer<DeserializeError>>(s.into_deserializer())
}

trait ValidateDtoExt<T> {
    fn validate<U, F>(self, f: F) -> PipelineResult<Request<Validated, U>>
    where
        F: FnOnce(T) -> PipelineResult<U>;
}

impl<T> ValidateDtoExt<T> for Request<Dto, T> {
    fn validate<U, F>(self, f: F) -> PipelineResult<Request<Validated, U>>
    where
        F: FnOnce(T) -> PipelineResult<U>,
    {
        self.try_advance_with(f)
    }
}

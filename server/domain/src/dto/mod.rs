mod register;
mod sendmessage;
mod auth_challenge;
mod signed_token;

use pipeline::error::PipelineResult;
use pipeline::request::Request;
use pipeline::stages::Dto;
use pipeline::stages::Validated;
pub use register::*;
pub use sendmessage::*;
pub use auth_challenge::*;

use serde::de::DeserializeOwned;
use serde::de::IntoDeserializer;
use serde::de::value::{Error as DeserializeError, StringDeserializer};

fn decode<T: DeserializeOwned>(s: String) -> Result<T, DeserializeError> {
    T::deserialize::<StringDeserializer<DeserializeError>>(s.into_deserializer())
}

trait ValidateDtoExt<T> {
    fn validate<U, F>(self, f: F) -> PipelineResult<Request<Validated, U>>
    where F: FnOnce(T) -> PipelineResult<U>;
}

impl<T> ValidateDtoExt<T> for Request<Dto, T> {
    fn validate<U, F>(self, f: F) -> PipelineResult<Request<Validated, U>>
    where F: FnOnce(T) -> PipelineResult<U>
    {
        self.try_advance_with(f)
    }
}
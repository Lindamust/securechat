use crate::{Bytes32, Bytes64};
use serde::Deserialize;
use serde::de::IntoDeserializer;
use std::str::FromStr;

impl AsRef<[u8]> for Bytes32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for Bytes64 {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl From<Vec<u8>> for Bytes32 {
    fn from(value: Vec<u8>) -> Self {
        Self::try_from(value).expect("msg")
    }
}

impl TryFrom<Vec<u8>> for Bytes64 {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let arr: [u8; 64] = value.as_slice().try_into()?;
        Ok(Self(arr))
    }
}

impl AsRef<[u8]> for Bytes64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBytes32Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBytes64Error;

impl FromStr for Bytes32 {
    type Err = ParseBytes32Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
            .map_err(|_: serde::de::value::Error| ParseBytes32Error)
    }
}

impl FromStr for Bytes64 {
    type Err = ParseBytes64Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
            .map_err(|_: serde::de::value::Error| ParseBytes64Error)
    }
}

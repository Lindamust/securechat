use serde::Deserialize;

use super::{Bytes32, Bytes64};
use crate::typestate::error::PipelineError;

use serde::de::IntoDeserializer;
use std::str::FromStr;

impl FromStr for Bytes32 {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer()).map_err(|_: serde::de::value::Error| {
            PipelineError::Internal("Bytes32 FromStr fail".to_owned())
        })
    }
}

impl FromStr for Bytes64 {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer()).map_err(|_: serde::de::value::Error| {
            PipelineError::Internal("Bytes64 FromStr fail".to_owned())
        })
    }
}

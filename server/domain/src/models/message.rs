use pipeline::error::PipelineError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent(String);

impl MessageContent {
    const MAX_BYTES: usize = 4_096;

    pub fn parse(raw: impl Into<String>) -> Result<Self, PipelineError> {
        let s = raw.into();

        if s.trim().is_empty() {
            return Err(PipelineError::Validation(
                "message cannot be empty".to_owned(),
            ));
        }

        if s.len() > Self::MAX_BYTES {
            return Err(PipelineError::Validation(format!(
                "message exceeds {} bytes",
                Self::MAX_BYTES
            )));
        }

        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

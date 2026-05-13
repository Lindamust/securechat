use pipeline::error::PipelineError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    const MIN: usize = 3;
    const MAX: usize = 32;

    pub fn parse(raw: impl Into<String>) -> Result<Self, PipelineError> {
        let s = raw.into().trim().to_owned();

        if s.len() < Self::MIN || s.len() > Self::MAX {
            return Err(PipelineError::Validation(format!(
                "username must be {MIN}–{MAX} characters",
                MIN = Self::MIN,
                MAX = Self::MAX
            )));
        }

        if !s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(PipelineError::Validation(
                "username may only contain letters, digits, underscores, or hyphens".to_owned(),
            ));
        }

        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

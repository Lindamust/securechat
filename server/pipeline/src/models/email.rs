use crate::error::PipelineError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(raw: impl Into<String>) -> Result<Self, PipelineError> {
        let s = raw.into().trim().to_lowercase();

        // Minimal RFC-5322 sanity check — use a proper library in prod.
        let at = s
            .find('@')
            .ok_or_else(|| PipelineError::Validation("email must contain '@'".to_owned()))?;

        let domain = &s[at + 1..];
        if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
            return Err(PipelineError::Validation(
                "email domain is invalid".to_owned(),
            ));
        }

        if s.len() > 254 {
            return Err(PipelineError::Validation(
                "email address is too long".to_owned(),
            ));
        }

        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

use serde::{Deserialize, Serialize};
use nutype::nutype;

#[nutype(
    sanitize(trim),
    validate(predicate = |s| validate_username(s).is_ok()),
    derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsRef)
)]
pub struct Username(String);

#[derive(Debug, thiserror::error)]
pub enum UsernameError {
    #[error("username must be 3-32 characters")]
    Length,

    #[error("username may only contain letters, digits, underscores, or hyphens")]
    InvalidChars,
}

fn validate_username(s: &str) -> Result<(), UsernameError> {
        let s = raw.into().trim();

        if s.len() < Username::MIN || s.len() > Username::MAX {
            return Err(UsernameError::Length);
        }

        if !s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(UsernameError::InvalidChars);
        }

        Ok()
}

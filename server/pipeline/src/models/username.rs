#[nutype::nutype(
    sanitize(trim),
    validate(with = validate_username, error = NameError),
    derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsRef)
)]
pub struct Username(String);

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("username must be 3-32 characters")]
    Length,

    #[error("username may only contain letters, digits, underscores, or hyphens")]
    InvalidChars,
}

fn validate_username(s: &str) -> Result<(), NameError> {
        if s.len() < 3 || s.len() > 32 {
            return Err(NameError::Length);
        }

        if !s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(NameError::InvalidChars);
        }

        Ok(())
}

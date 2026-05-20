use crate::error::PipelineError;

/// Holds a password that has passed the strength check but has NOT yet been
/// hashed. The `Drop` impl zeroes the allocation as a best-effort defence.
#[derive(Debug)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn parse(raw: impl Into<String>) -> Result<Self, PipelineError> {
        let s = raw.into();

        if s.len() < 8 {
            return Err(PipelineError::Validation(
                "password must be at least 8 characters".to_owned(),
            ));
        }

        // zxcvbn gives a realistic strength score 0–4.
        let estimate = zxcvbn::zxcvbn(&s, &[]);
        let score = estimate.score();

        if score < zxcvbn::Score::Two {
            let feedback = estimate
                .feedback()
                .and_then(|f| f.warning())
                .map(|w| w.to_string())
                .unwrap_or_else(|| "try a longer or more random password".to_owned());

            return Err(PipelineError::WeakPassword(score as u8, feedback));
        }

        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Drop for PlainPassword {
    fn drop(&mut self) {
        // Zero the backing memory.
        // Safety: we own the String and are about to drop it.
        for byte in unsafe { self.0.as_bytes_mut() } {
            *byte = 0;
        }
    }
}

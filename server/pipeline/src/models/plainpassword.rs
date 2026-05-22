use serde::Deserialize;

fn validate_password_strength(s: &str) -> bool {
    if s.len() < 8 { return false; }
    let estimate = zxcvbn::zxcvbn(s, &[]);
    estimate.score() >= zxcvbn::Score::Two
}

#[nutype(
    validate(predicate = validate_password_strength),
    derive(Debug, Deserialize)
    // Intentionally NOT Serialize or Clone because passwords must never appear in
    // response bodies and should not be copied around freely.
)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for PlainPassword {
    fn as_ref(&self) -> &str {
        // Safety: PlainPassword(String) is repr(transparent) over String.
        unsafe {
            let raw: *const PlainPassword = self;
            let s: *const String = raw as *const String;
            &*s
        }
    }
}

impl Drop for PlainPassword {
    /// Best-effort zero of the password bytes before deallocation.
    fn drop(&mut self) {
        unsafe {
            let raw: *mut PlainPassword = self;
            let s: *mut String = raw as *mut String;
            for byte in (*s).as_bytes_mut() {
                *byte = 0;
            }
        }
    }
}


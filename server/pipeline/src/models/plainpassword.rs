fn validate_password_strength(s: &str) -> bool {
    if s.len() < 8 { return false; }
    let estimate = zxcvbn::zxcvbn(s, &[]);
    estimate.score() >= zxcvbn::Score::Two
}

#[nutype::nutype(
    validate(predicate = validate_password_strength),
    derive(Debug, Deserialize),
    // Intentionally NOT Serialize or Clone because passwords must never appear in
    // response bodies and should not be copied around freely.
    derive_unchecked(zeroize::ZeroizeOnDrop),
    new_unchecked
)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn new(s: String) -> Result<Self, nutype::error::Error> {
        if validate_password_strength(&s) {
            Ok(unsafe {
                PlainPassword::new_unchecked(s)
            })
        } else {
            Err( nutype::error::Error::ValidationError)
        }
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


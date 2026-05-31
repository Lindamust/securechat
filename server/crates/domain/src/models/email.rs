use nutype::nutype;

#[nutype(
    sanitize(trim, lowercase),
    validate(predicate = validate_email_str),
    derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsRef)
)]
pub struct Email(String);

fn validate_email_str(s: &str) -> bool {
    let s = s.trim();
    if s.len() > 254 {
        return false;
    };
    let Some(at) = s.find('@') else { return false };
    let domain = &s[at + 1..];
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

impl Email {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: &'static str,
    pub index: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ValidationErrors {
    pub errors: Vec<FieldError>,
}

pub type ValidationResult<T> = Result<T, ValidationErrors>;

impl ValidationErrors {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn push(&mut self, err: FieldError) {
        self.errors.push(err);
    }

    pub fn extend(&mut self, other: ValidationErrors) {
        self.errors.extend(other.errors);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn prefixed(mut self, field: &'static str) -> Self {
        for err in &mut self.errors {
            err.field = field;
        }
        self
    }
}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for err in &self.errors {
            if let Some(i) = err.index {
                writeln!(f, "{}[{}]: {}", err.field, i, err.message)?;
            } else {
                writeln!(f, "{}: {}", err.field, err.message)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_push_and_display() {
        let mut errors = ValidationErrors::new();

        errors.push(FieldError {
            field: "email",
            index: None,
            message: "invalid".into(),
        });

        errors.push(FieldError {
            field: "items",
            index: Some(2),
            message: "bad".into(),
        });

        let output = format!("{}", errors);

        assert!(output.contains("email: invalid"));
        assert!(output.contains("items[2]: bad"));
    }

    #[test]
    fn error_extend() {
        let mut a = ValidationErrors::new();
        let mut b = ValidationErrors::new();

        a.push(FieldError {
            field: "a",
            index: None,
            message: "err_A".into(),
        });
        b.push(FieldError {
            field: "b",
            index: Some(3),
            message: "err_B".into(),
        });

        a.extend(b);
        assert_eq!(a.errors.len(), 2);

        let output = format!("{}", a);

        assert!(output.contains("a: err_A"));
        assert!(output.contains("b[3]: err_B"));
    }
}

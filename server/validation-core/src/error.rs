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

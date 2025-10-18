use std::fmt::Display;

pub trait DomainError {
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;
}

#[derive(Debug)]
pub enum SharedError {
    NotAvailable,
}

impl DomainError for SharedError {
    fn code(&self) -> &'static str {
        match self {
            Self::NotAvailable => "error.shared.not_available",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Self::NotAvailable => "This is functionality is not available.",
        }
    }
}

impl Display for SharedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

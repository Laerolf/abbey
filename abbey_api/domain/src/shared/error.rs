use std::fmt::Display;

/// Represents an [Error][`std::error::Error`] regarding the domain of this project.
pub trait DomainError: std::error::Error + std::fmt::Debug {
    /// Gets the locale code of this [`DomainError`].
    fn code(&self) -> &'static str;

    /// Gets the message of this [`DomainError`].
    fn message(&self) -> &'static str;
}

/// Represents a [`DomainError`] shared in the domain of this project.
#[derive(Debug)]
pub enum SharedError {
    NotAvailable,
}

impl std::error::Error for SharedError {}

impl DomainError for SharedError {
    /// Gets the locale code of this [`SharedError`].
    fn code(&self) -> &'static str {
        match self {
            Self::NotAvailable => "error.shared.not_available",
        }
    }

    /// Gets the locale code of this [`SharedError`].
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

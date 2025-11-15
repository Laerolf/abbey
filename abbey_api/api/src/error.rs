use std::fmt::Display;

/// Represents an API [Error][`std::error::Error`].
#[derive(Debug)]
pub enum ApiError {
    MissingHost,
    MissingPort,
    MissingDbUrl,
    InvalidDbUrl,
}

impl ApiError {
    /// Gets the locale code of this [`ApiError`].
    fn code(&self) -> &'static str {
        match self {
            Self::MissingHost => "error.api.missing_host",
            Self::MissingPort => "error.api.missing_port",
            Self::MissingDbUrl => "error.api.missing_db_url",
            Self::InvalidDbUrl => "error.api.invalid_db_url",
        }
    }

    /// Gets the locale code of this [`ApiError`].
    fn message(&self) -> &'static str {
        match self {
            Self::MissingHost => "A host is required.",
            Self::MissingPort => "A port is required.",
            Self::MissingDbUrl => "A database connection url is required.",
            Self::InvalidDbUrl => "Failed to create a database connection.",
        }
    }
}

impl std::error::Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

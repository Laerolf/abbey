use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::shared::error::{DomainError, DomainErrorKind};
use serde::Serialize;

/// Represents an API [Error][`std::error::Error`].
#[derive(Debug)]
pub enum StartupError {
    MissingHost,
    MissingPort,
    MissingDbUrl,
    InvalidDbUrl,
}

impl StartupError {
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

impl Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message(), self.code())
    }
}

#[derive(Serialize)]
pub struct AppError {
    pub message: String,
    pub code: String,
}

impl<K> From<DomainError<K>> for AppError
where
    K: DomainErrorKind,
{
    fn from(value: DomainError<K>) -> Self {
        Self {
            code: value.kind().code(),
            message: value.kind().message(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

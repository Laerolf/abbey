use std::{collections::HashMap, fmt::Display};

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};

pub struct DomainError<K> {
    kind: K,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    context: Option<HashMap<String, String>>,
}

impl<K> DomainError<K>
where
    K: DomainErrorKind,
{
    pub fn from(kind: K) -> Self {
        Self {
            kind,
            cause: None,
            context: None,
        }
    }

    pub fn with_cause(mut self, cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn kind(&self) -> &K {
        &self.kind
    }

    pub fn context(&self) -> &Option<HashMap<String, String>> {
        &self.context
    }
}

impl<K> std::error::Error for DomainError<K> where K: DomainErrorKind + Send + Sync {}

impl<K> std::fmt::Debug for DomainError<K>
where
    K: DomainErrorKind,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("DomainError");
        debug.field("code", &self.kind.code());
        debug.field("message", &self.kind.message());
        debug.field("cause", &self.cause);
        debug.field("context", &self.context);
        debug.finish()
    }
}

impl<K> Display for DomainError<K>
where
    K: DomainErrorKind + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.kind().code(), self.kind().message())
    }
}

impl<K> IntoResponse for DomainError<K>
where
    K: DomainErrorKind,
{
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(self.kind.http_status())
            .body(Body::empty())
            .unwrap()
    }
}

/// Represents the kind of a [`DomainError`].
pub trait DomainErrorKind {
    /// Gets the locale code of this [`DomainErrorKind`].
    fn code(&self) -> String;

    /// Gets the message of this [`DomainError`].
    fn message(&self) -> String;

    /// Gets the HTTP status code of this [`DomainErrorKind`].
    fn http_status(&self) -> StatusCode;

    /// Gets the unknown error of this [`DomainErrorKind`].
    fn unknown() -> Self;
}

#[derive(Debug)]
pub enum SharedErrorKind {
    NotAvailable,
    Unknown,
}

impl DomainErrorKind for SharedErrorKind {
    fn code(&self) -> String {
        match self {
            Self::NotAvailable => "error.shared.not_available".to_string(),
            Self::Unknown => "error.shared.unknown".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::NotAvailable => "This is functionality is not available.".to_string(),
            Self::Unknown => "An unknown error occurred.".to_string(),
        }
    }

    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl Display for SharedErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

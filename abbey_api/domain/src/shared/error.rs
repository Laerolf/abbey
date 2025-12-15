use std::fmt::Display;

pub struct DomainError<K> {
    kind: K,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl<K> DomainError<K>
where
    K: DomainErrorKind,
{
    pub fn from(kind: K) -> Self {
        Self { kind, cause: None }
    }

    pub fn with_cause(mut self, cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    pub fn kind(&self) -> &K {
        &self.kind
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
        if let Some(cause) = &self.cause {
            debug.field("cause", cause);
        }
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

/// Represents the kind of a [`DomainError`].
pub trait DomainErrorKind {
    /// Gets the locale code of this [`DomainError`].
    fn code(&self) -> String;

    /// Gets the message of this [`DomainError`].
    fn message(&self) -> String;
}

#[derive(Debug)]
pub enum SharedErrorKind {
    NotAvailable,
}

impl DomainErrorKind for SharedErrorKind {
    fn code(&self) -> String {
        match self {
            Self::NotAvailable => "error.shared.not_available".to_string(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::NotAvailable => "This is functionality is not available.".to_string(),
        }
    }
}

impl Display for SharedErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

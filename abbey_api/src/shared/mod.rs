use crate::shared::error::DomainError;

pub mod error;

/// Represents a factory related to a topic in the domain.
pub trait DomainFactory<T> {
    /// Runs the factory logic.
    fn run() -> Result<T, Box<dyn DomainError>>;
}

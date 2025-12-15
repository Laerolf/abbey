use crate::shared::error::DomainErrorKind;

pub mod db;
pub mod error;

/// Represents a factory related to a topic in the domain.
pub trait DomainFactory<T> {
    /// Runs the factory logic.
    fn run(&self) -> Result<T, Box<dyn DomainErrorKind>>;
}

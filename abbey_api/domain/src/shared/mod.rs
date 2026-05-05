use time::OffsetDateTime;

use crate::shared::error::{DomainError, DomainErrorKind};

pub mod error;

/// Represents a persisted element.
pub trait DomainElement<E: DomainErrorKind> {
    /// Gets the ID of this [`DomainElement`].
    fn id(&self) -> Result<i32, DomainError<E>>;

    /// Gets the [creation date][`OffsetDateTime`] of this [`DomainElement`].
    fn created_at(&self) -> &Option<OffsetDateTime>;

    /// Gets the [latest update date][`OffsetDateTime`] of this [`DomainElement`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime>;
}

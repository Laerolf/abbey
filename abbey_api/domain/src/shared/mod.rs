use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use utoipa::ToSchema;

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

/// Represents a duration.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, PartialEq)]
pub struct DurationDto {
    /// The seconds of the duration.
    pub seconds: i64,
    /// The nanoseconds of the duration.
    pub nanoseconds: i32,
}

impl From<Duration> for DurationDto {
    fn from(duration: Duration) -> Self {
        Self {
            seconds: duration.whole_seconds(),
            nanoseconds: duration.subsec_nanoseconds(),
        }
    }
}

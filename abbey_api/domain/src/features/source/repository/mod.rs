use entity::sources;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};
use tracing::warn;

use crate::{features::source::error::SourceErrorKind, shared::error::DomainError};

/// Represents an element that handles all [Source][`super::domain::Source`] database topics.
#[derive(Default, Clone)]
pub struct SourceRepository;

impl SourceRepository {
    /// Gets [`Sources`][Vec<sources::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<sources::Model>, DomainError<SourceErrorKind>> {
        sources::Entity::find()
            .filter(sources::Column::Id.is_in(ids.to_vec()))
            .order_by_asc(sources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::GetByIds).with_cause(error))
    }

    /// Creates [`Sources`][`Vec<sources::Model>`] and persists them in the database.
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        models: Vec<sources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<sources::Model>, DomainError<SourceErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        sources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))
    }
}

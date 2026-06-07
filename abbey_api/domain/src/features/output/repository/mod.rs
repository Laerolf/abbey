use entity::resources;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use tracing::warn;

use crate::{features::output::error::ResourceErrorKind, shared::error::DomainError};

/// Represents an element that handles all [Resource][`super::domain::resource::Resource`] database topics.
#[derive(Default, Clone)]
pub struct ResourceRepository;

impl ResourceRepository {
    /// Gets all [`Resources`][Vec<resources::Model>].
    pub async fn get_all<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        resources::Entity::find()
            .distinct()
            .order_by_asc(resources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::GetAll).with_cause(error))
    }

    /// Gets the [`Resources`][Vec<resources::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        resources::Entity::find()
            .filter(resources::Column::Id.is_in(ids.to_vec()))
            .distinct()
            .order_by_asc(resources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::GetByIds).with_cause(error))
    }

    /// Gets the [`Resources`][Vec<resources::Model>] with the provided names.
    pub async fn get_by_names<C: ConnectionTrait>(
        &self,
        names: &[String],
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        resources::Entity::find()
            .filter(resources::Column::Name.is_in(names.to_vec()))
            .distinct()
            .order_by_asc(resources::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::GetByNames).with_cause(error))
    }

    /// Creates new [`Resources`][Vec<resources::Model>].
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        models: Vec<resources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<resources::Model>, DomainError<ResourceErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        resources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ResourceErrorKind::Creation).with_cause(error))
    }
}

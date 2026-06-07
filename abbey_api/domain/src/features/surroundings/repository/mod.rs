use entity::{surroundings, surroundings_sources};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::warn;

use crate::{features::surroundings::error::SurroundingsErrorKind, shared::error::DomainError};

/// Represents an element that handles all [Surroundings][`super::domain::Surroundings`] database topics.
#[derive(Default, Clone)]
pub struct SurroundingsRepository;

impl SurroundingsRepository {
    /// Finds a [Surroundings][`surroundings::Model`] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<surroundings::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::FindById).with_cause(error))
    }

    /// Gets the [Surroundings][`Vec<surroundings::Model>`] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<surroundings::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings::Entity::find()
            .filter(surroundings::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::GetByIds).with_cause(error))
    }

    /// Gets all the [Sources][Vec<Source>] for the provided Surroundings ID.
    pub async fn get_all_sources_by_surroundings_id<C: ConnectionTrait>(
        &self,
        surroundings_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<surroundings_sources::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings_sources::Entity::find()
            .filter(surroundings_sources::Column::SurroundingsId.eq(*surroundings_id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })
    }

    /// Gets all the [Sources][Vec<Source>] for the provided Surroundings IDs.
    pub async fn get_all_sources_by_surroundings_ids<C: ConnectionTrait>(
        &self,
        surroundings_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<surroundings_sources::Model>, DomainError<SurroundingsErrorKind>> {
        surroundings_sources::Entity::find()
            .filter(surroundings_sources::Column::SurroundingsId.is_in(surroundings_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })
    }

    /// Creates a new [`Surroundings`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: surroundings::ActiveModel,
        db_connection: &C,
    ) -> Result<surroundings::Model, DomainError<SurroundingsErrorKind>> {
        surroundings::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))
    }

    /// Assigns Sources to a [`Surroundings`].
    pub async fn assign_sources_to_surroundings<C: ConnectionTrait>(
        &self,
        models: Vec<surroundings_sources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<surroundings_sources::Model>, DomainError<SurroundingsErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        surroundings_sources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))
    }
}

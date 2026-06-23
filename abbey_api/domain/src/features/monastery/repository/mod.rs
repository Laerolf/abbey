use entity::{monasteries, monastery_monks};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::warn;

use crate::{features::monastery::error::MonasteryErrorKind, shared::error::DomainError};

/// Represents an element that handles all [Monastery][`super::domain::Monastery`] database topics.
#[derive(Default, Clone)]
pub struct MonasteryRepository;

impl MonasteryRepository {
    /// Finds a [`Monastery`][monasteries::Model] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<monasteries::Model>, DomainError<MonasteryErrorKind>> {
        monasteries::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::FindById).with_cause(error))
    }

    /// Gets the [`Monasteries`][Vec<monasteries::Model>] with the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<monasteries::Model>, DomainError<MonasteryErrorKind>> {
        monasteries::Entity::find()
            .filter(monasteries::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::GetByIds).with_cause(error))
    }

    /// Gets all [`Monastery Monks`][Vec<monastery_monks::Model>] with the provided Monastery ID.
    pub async fn get_monastery_monks_by_monastery_id<C: ConnectionTrait>(
        &self,
        monastery_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<monastery_monks::Model>, DomainError<MonasteryErrorKind>> {
        monastery_monks::Entity::find()
            .filter(monastery_monks::Column::MonasteryId.eq(*monastery_id))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error))
    }

    /// Gets all [`Monastery Monks`][Vec<monastery_monks::Model>] with the provided Monastery IDs.
    pub async fn get_monastery_monks_by_monastery_ids<C: ConnectionTrait>(
        &self,
        monastery_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<monastery_monks::Model>, DomainError<MonasteryErrorKind>> {
        monastery_monks::Entity::find()
            .filter(monastery_monks::Column::MonasteryId.is_in(monastery_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error))
    }

    /// Creates a new [`Monastery`].
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: monasteries::ActiveModel,
        db_connection: &C,
    ) -> Result<monasteries::Model, DomainError<MonasteryErrorKind>> {
        model
            .insert(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))
    }

    /// Assigns Monks to a [`Monastery`].
    pub async fn assign_monks<C: ConnectionTrait>(
        &self,
        models: Vec<monastery_monks::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<monastery_monks::Model>, DomainError<MonasteryErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        monastery_monks::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))
    }
}

use entity::{monk_skills, monks};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::warn;

use crate::{features::actor::error::ActorErrorKind, shared::error::DomainError};

/// Represents an element that handles all [`Monk`] database topics.
#[derive(Default, Clone)]
pub struct MonkRepository;

impl MonkRepository {
    /// Finds a [`Monk`][monks::Model] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<monks::Model>, DomainError<ActorErrorKind>> {
        monks::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))
    }

    /// Gets the [`Monks`][Vec<monks::Model>] with the provided Monk IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<monks::Model>, DomainError<ActorErrorKind>> {
        monks::Entity::find()
            .filter(monks::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByIds).with_cause(error))
    }

    /// Gets the [`Monk skills`][Vec<monk_skills::Model>] with the provided Monk ID.
    pub async fn get_skill_assignments_by_monk_id<C: ConnectionTrait>(
        &self,
        monk_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<monk_skills::Model>, DomainError<ActorErrorKind>> {
        monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.eq(*monk_id))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })
    }

    /// Gets all [`Monk skills`][Vec<monk_skills::Model>] with the provided Monk IDs.
    pub async fn get_skill_assignments_by_monk_ids<C: ConnectionTrait>(
        &self,
        monk_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<monk_skills::Model>, DomainError<ActorErrorKind>> {
        monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(monk_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })
    }

    /// Gets [`Monks`][Vec<monks::Model>] with the provided Process ID.
    pub async fn get_by_process_id<C: ConnectionTrait>(
        &self,
        process_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<monks::Model>, DomainError<ActorErrorKind>> {
        monks::Entity::find()
            .filter(monks::Column::AssignedCyclicProcessId.eq(*process_id))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByProcessId).with_cause(error))
    }

    /// Gets [`Monks`][Vec<monks::Model>] with the provided Process IDs.
    pub async fn get_by_process_ids<C: ConnectionTrait>(
        &self,
        process_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<monks::Model>, DomainError<ActorErrorKind>> {
        monks::Entity::find()
            .filter(monks::Column::AssignedCyclicProcessId.is_in(process_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByProcessIds).with_cause(error))
    }

    /// Assigns Skills to Monks.
    pub async fn assign_skills<C: ConnectionTrait>(
        &self,
        models: Vec<monk_skills::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<monk_skills::Model>, DomainError<ActorErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        monk_skills::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))
    }

    /// Creates many [`Monk`][`Vec<monks::Model>`]s and persists them in the database.
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        models: Vec<monks::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<monks::Model>, DomainError<ActorErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        monks::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))
    }

    /// Updates a [`Monk`][monks::ActiveModel].
    pub async fn update<C: ConnectionTrait>(
        &self,
        model: monks::ActiveModel,
        db_connection: &C,
    ) -> Result<monks::Model, DomainError<ActorErrorKind>> {
        monks::Entity::update(model)
            .exec(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Update).with_cause(error))
    }
}

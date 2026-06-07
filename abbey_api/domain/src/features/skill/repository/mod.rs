use entity::skills;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use tracing::warn;

use crate::{features::skill::error::SkillErrorKind, shared::error::DomainError};

/// Represents an element that handles all [`Skill`][`super::domain::Skill`] database topics.
#[derive(Default, Clone)]
pub struct SkillRepository;

impl SkillRepository {
    /// Gets all [`Skills`][Vec<skills::Model>].
    pub async fn get_all<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<skills::Model>, DomainError<SkillErrorKind>> {
        skills::Entity::find()
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::GetAll).with_cause(error))
    }

    /// Gets all [`Skills`][Vec<skills::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: Vec<i32>,
        db_connection: &C,
    ) -> Result<Vec<skills::Model>, DomainError<SkillErrorKind>> {
        skills::Entity::find()
            .filter(skills::Column::Id.is_in(ids))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::GetByIds).with_cause(error))
    }

    /// Finds [`Skills`][Vec<skills::Model>] with the provided names.
    pub async fn get_by_names<C: ConnectionTrait>(
        &self,
        names: &[String],
        db_connection: &C,
    ) -> Result<Vec<skills::Model>, DomainError<SkillErrorKind>> {
        skills::Entity::find()
            .filter(skills::Column::Name.is_in(names))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::FindByNames).with_cause(error))
    }

    /// Creates many [`Skills`][Vec<skills::Model>].
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        models: Vec<skills::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<skills::Model>, DomainError<SkillErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        skills::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::Creation).with_cause(error))
    }
}

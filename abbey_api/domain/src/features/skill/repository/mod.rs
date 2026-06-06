use entity::skills;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use tracing::warn;

use crate::{
    features::skill::{
        domain::Skill, error::SkillErrorKind, forms::SkillCreationForm, mapper::SkillMapper,
    },
    shared::error::DomainError,
};

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

    /// Finds a [`Skill`] by its name.
    pub async fn find_by_name<C: ConnectionTrait>(
        &self,
        name: &String,
        db_connection: &C,
    ) -> Result<Option<Skill>, DomainError<SkillErrorKind>> {
        let Some(skill_model) = skills::Entity::find()
            .filter(skills::Column::Name.eq(name))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::FindByName).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(SkillMapper::to_domain_entity(skill_model)))
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

    /// Creates a [`Skill`] and persists it in the database.
    pub async fn create(
        &self,
        form: SkillCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Skill, DomainError<SkillErrorKind>> {
        let new_skill_model: skills::Model = SkillMapper::to_new_active_model(form)
            .insert(db_transaction)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::Creation).with_cause(error))?;

        Ok(SkillMapper::to_domain_entity(new_skill_model))
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

    /// Finds a [`Skill`] by its name or creates it if it doesn't exist.
    pub async fn find_by_name_or_create(
        &self,
        form: SkillCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Skill, DomainError<SkillErrorKind>> {
        match self.find_by_name(&form.name, db_transaction).await? {
            Some(skill) => Ok(skill),
            None => self.create(form, db_transaction).await,
        }
    }
}

use std::collections::HashSet;

use entity::skills;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect,
};

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
    pub async fn get_all<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let all_skills = skills::Entity::find()
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::GetAll).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        Ok(all_skills)
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

    /// Finds [`Skills`][Vec<Skill>] by their names.
    pub async fn find_many_by_name<C: ConnectionTrait>(
        &self,
        names: &Vec<String>,
        db_connection: &C,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let skills = skills::Entity::find()
            .filter(skills::Column::Name.is_in(names))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::FindByNames).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        Ok(skills)
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

    /// Creates many [`Skills`][Vec<Skill>].
    pub async fn create_many(
        &self,
        forms: Vec<SkillCreationForm>,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let new_skill_active_models: Vec<skills::ActiveModel> = forms
            .into_iter()
            .map(SkillMapper::to_new_active_model)
            .collect();

        let new_skill_models = skills::Entity::insert_many(new_skill_active_models)
            .exec_with_returning_many(db_transaction)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::Creation).with_cause(error))?;

        Ok(new_skill_models
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect())
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

    /// Finds many [`Skills`][`Vec<Skill>`] by their name or creates them if they don't exist.
    pub async fn find_by_name_or_create_many(
        &self,
        forms: Vec<SkillCreationForm>,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let skill_names = forms.iter().map(|form| form.name.clone()).collect();

        let found_skill_models = self.find_many_by_name(&skill_names, db_transaction).await?;

        let found_skill_names: HashSet<String> = found_skill_models
            .iter()
            .map(|skill| skill.name().to_string())
            .collect();

        let missing_skill_forms: Vec<SkillCreationForm> = forms
            .into_iter()
            .filter(|form| !found_skill_names.contains(&form.name))
            .collect();

        let created_skill_models = if !missing_skill_forms.is_empty() {
            self.create_many(missing_skill_forms, db_transaction)
                .await?
        } else {
            Vec::new()
        };

        let mut all_skills = found_skill_models;
        all_skills.extend(created_skill_models);

        Ok(all_skills)
    }
}

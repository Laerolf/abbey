use std::collections::HashSet;

use entity::skills;
use sea_orm::ConnectionTrait;

use crate::{
    features::skill::{
        domain::Skill, error::SkillErrorKind, forms::SkillBlueprint, mapper::SkillMapper,
        repository::SkillRepository,
    },
    shared::error::DomainError,
};

/// Represents a command service for [`Skills`][Skill].
#[derive(Clone)]
pub struct SkillCommandService {
    repository: SkillRepository,
}

impl SkillCommandService {
    /// Creates a new [`SkillCommandService`].
    pub fn new(repository: SkillRepository) -> Self {
        Self { repository }
    }

    /// Creates many [`Skills`][Vec<Skills>].
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        blueprints: Vec<SkillBlueprint>,
        db_connection: &C,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let active_models: Vec<skills::ActiveModel> = blueprints
            .into_iter()
            .map(SkillMapper::to_new_active_model)
            .collect();

        let models = self
            .repository
            .create_many(active_models, db_connection)
            .await?;

        Ok(models
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect())
    }

    /// Gets [`Skills`][Vec<Skill>] with the provided names or creates them.
    pub async fn get_by_names_or_create<C: ConnectionTrait>(
        &self,
        names: &[String],
        db_connection: &C,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let blueprints: Vec<SkillBlueprint> = names.iter().map(SkillBlueprint::new).collect();

        let existing_models = self.repository.get_by_names(names, db_connection).await?;

        let found_skill_names: HashSet<String> = existing_models
            .clone()
            .into_iter()
            .map(|skill| skill.name)
            .collect();

        let missing_models: Vec<SkillBlueprint> = blueprints
            .into_iter()
            .filter(|blueprint| !found_skill_names.contains(&blueprint.name))
            .collect();

        let created_models = self.create_many(missing_models, db_connection).await?;

        Ok(existing_models
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .chain(created_models)
            .collect())
    }
}

/// Represents a query service for [`Skills`][Skill].
#[derive(Clone)]
pub struct SkillQueryService {
    repository: SkillRepository,
}

impl SkillQueryService {
    /// Creates a new [`SkillQueryService`].
    pub fn new(repository: SkillRepository) -> Self {
        Self { repository }
    }

    /// Gets all [`Skills`][Vec<Skill>] matching the provided IDs.
    pub async fn get_all_by_ids<C: ConnectionTrait>(
        &self,
        ids: Vec<i32>,
        db_connection: &C,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        Ok(self
            .repository
            .get_by_ids(ids, db_connection)
            .await?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect())
    }
}

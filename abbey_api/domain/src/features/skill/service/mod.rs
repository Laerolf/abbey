use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    features::skill::{
        domain::Skill, error::SkillErrorKind, forms::SkillCreationForm, mapper::SkillMapper,
        repository::SkillRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Skill`] topic.
#[derive(Clone)]
pub struct SkillService {
    repository: SkillRepository,
}

impl SkillService {
    /// Creates a new [`SkillService`].
    pub fn new(repository: SkillRepository) -> Self {
        Self { repository }
    }

    /// Finds many [`Skills`][Vec<Skill>] with the provided names or creates them.
    pub async fn find_many_by_name_or_create(
        &self,
        names: Vec<impl Into<String>>,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let creation_forms = names
            .into_iter()
            .map(|skill_name| SkillCreationForm::new(skill_name.into()))
            .collect();

        self.repository
            .find_by_name_or_create_many(creation_forms, db_transaction)
            .await
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

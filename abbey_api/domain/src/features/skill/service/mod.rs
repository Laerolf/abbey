use sea_orm::DatabaseTransaction;

use crate::{
    features::skill::{
        domain::Skill, error::SkillErrorKind, forms::SkillCreationForm, mapper::SkillMapper,
        repository::SkillRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Skill`] topic.
#[derive(Default, Clone)]
pub struct SkillService {
    repository: SkillRepository,
}

impl SkillService {
    /// Finds a [`Skill`] by its name, if not found, the skill will be created.
    pub async fn find_by_name_or_create(
        &self,
        name: impl Into<String>,
    ) -> Result<Skill, DomainError<SkillErrorKind>> {
        let creation_form = SkillCreationForm::new(name);

        let model = self
            .repository
            .find_by_name_or_create(creation_form)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::Creation).with_cause(error))?;

        Ok(SkillMapper::to_domain_entity(model))
    }

    /// Creates a new [`Skill`].
    pub async fn create_skill(
        &self,
        name: impl Into<String>,
    ) -> Result<Skill, DomainError<SkillErrorKind>> {
        let creation_form = SkillCreationForm::new(name);

        let model = self
            .repository
            .create(creation_form)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::Creation).with_cause(error))?;

        Ok(SkillMapper::to_domain_entity(model))
    }

    /// Creates many [`Skills`][`Vec<Skill>`].
    pub async fn create_many_skills_in_transaction(
        &self,
        names: Vec<impl Into<String>>,
        transaction: &DatabaseTransaction,
    ) -> Result<Vec<Skill>, DomainError<SkillErrorKind>> {
        let creation_forms = names
            .into_iter()
            .map(|skill_name| SkillCreationForm::new(skill_name.into()))
            .collect();

        let skill_models = self
            .repository
            .create_many_in_transaction(creation_forms, transaction)
            .await
            .map_err(|error| DomainError::from(SkillErrorKind::Creation).with_cause(error))?;

        Ok(skill_models
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect())
    }
}

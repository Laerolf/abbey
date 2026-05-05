use sea_orm::DatabaseTransaction;

use crate::{
    features::skill::{
        domain::Skill, error::SkillErrorKind, forms::SkillCreationForm, repository::SkillRepository,
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

use sea_orm::DatabaseTransaction;

use crate::{
    features::{
        actor::{domain::monk::Monk, forms::MonkCreationForm, service::MonkService},
        monastery::{
            error::MonasteryErrorKind,
            forms::MonasteryCreationForm,
            repository::{MonasteryRepository, MonasteryWithRelations},
        },
        skill::{domain::Skill, service::SkillService},
    },
    shared::error::DomainError,
};

/// The default amount of [Monks][`Monk`] in [`Monastery`].
pub const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

/// Represents a service handling the [`Monastery`] topic.
#[derive(Clone)]
pub struct MonasteryService {
    repository: MonasteryRepository,
    monk_service: MonkService,
    skill_service: SkillService,
}

impl MonasteryService {
    /// Creates a new [`MonasteryService`].
    pub fn new(monk_service: MonkService, skill_service: SkillService) -> Self {
        Self {
            repository: MonasteryRepository::default(),
            monk_service,
            skill_service,
        }
    }

    /// Creates [Monks][`Monk`] for a new [`Monastery`].
    async fn create_monks_in_transaction(
        &self,
        transaction: &DatabaseTransaction,
    ) -> Result<Vec<Monk>, DomainError<MonasteryErrorKind>> {
        let skill_names = vec!["cooking", "brewing"];

        let skills: Vec<Skill> = self
            .skill_service
            .create_many_skills_in_transaction(skill_names, transaction)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))?;

        let skill_ids: Vec<i32> = skills.into_iter().map(|skill| skill.id).collect();

        let monk_creation_forms = (0..DEFAULT_AMOUNT_OF_MONKS)
            .map(|_| MonkCreationForm::new("Maurits", skill_ids.clone()))
            .collect();

        self.monk_service
            .create_many_monks_in_transaction(monk_creation_forms, transaction)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))
    }

    /// Creates a new [`Monastery`][`MonasteryWithRelations`].
    pub async fn create_monastery_in_transaction(
        &self,
        transaction: &DatabaseTransaction,
    ) -> Result<MonasteryWithRelations, DomainError<MonasteryErrorKind>> {
        let monks: Vec<Monk> = self.create_monks_in_transaction(transaction).await?;

        let monk_ids = monks.into_iter().map(|monk| monk.id).collect();

        let creation_form = MonasteryCreationForm::new(monk_ids);

        self.repository
            .create_with_relations(creation_form)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))
    }
}

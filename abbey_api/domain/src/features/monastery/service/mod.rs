use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    features::{
        actor::{
            domain::monk::Monk,
            forms::MonkCreationForm,
            service::{MonkQueryService, MonkService},
        },
        monastery::{
            domain::Monastery, error::MonasteryErrorKind, forms::MonasteryCreationForm,
            mapper::MonasteryMapper, repository::MonasteryRepository,
        },
        skill::{domain::Skill, service::SkillService},
    },
    shared::{DomainElement, error::DomainError},
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
    pub fn new(
        repository: MonasteryRepository,
        monk_service: MonkService,
        skill_service: SkillService,
    ) -> Self {
        Self {
            repository,
            monk_service,
            skill_service,
        }
    }

    /// Creates [Monks][`Monk`] for a new [`Monastery`].
    async fn create_monks(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Monk>, DomainError<MonasteryErrorKind>> {
        let skill_names = vec!["cooking", "brewing"];

        let skills: Vec<Skill> = self
            .skill_service
            .find_many_by_name_or_create(skill_names, db_transaction)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))?;

        let skill_ids: Vec<i32> = skills.iter().map(|skill| skill.id().unwrap()).collect();

        let monk_creation_forms = (0..DEFAULT_AMOUNT_OF_MONKS)
            .map(|_| MonkCreationForm::new("Maurits", skill_ids.clone()))
            .collect();

        self.monk_service
            .create_many_monks(monk_creation_forms, db_transaction)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))
    }

    /// Creates a new [`Monastery`].
    pub async fn create_monastery(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Monastery, DomainError<MonasteryErrorKind>> {
        let monks: Vec<Monk> = self.create_monks(db_transaction).await?;

        self.repository
            .create(MonasteryCreationForm::new(monks), db_transaction)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))
    }
}

/// Represents a query service for [`Monasteries`][Monastery].
#[derive(Clone)]
pub struct MonasteryQueryService {
    repository: MonasteryRepository,
    monk_query_service: MonkQueryService,
}

impl MonasteryQueryService {
    /// Creates a new [`MonasteryQueryService`].
    pub fn new(repository: MonasteryRepository, monk_query_service: MonkQueryService) -> Self {
        Self {
            repository,
            monk_query_service,
        }
    }

    /// Finds a [`Monastery`] with the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Monastery, DomainError<MonasteryErrorKind>> {
        let model = self
            .repository
            .find_by_id(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(MonasteryErrorKind::GetById))?;

        let monks = self
            .monk_query_service
            .get_all_by_monastery_id(&model.id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error)
            })?;

        MonasteryMapper::to_domain_entity(model, monks)
            .map_err(|error| DomainError::from(MonasteryErrorKind::Restore).with_cause(error))
    }
}

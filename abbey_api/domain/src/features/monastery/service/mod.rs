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

        let monk_ids: Vec<i32> = self
            .repository
            .get_monastery_monks_by_monastery_id(&model.id, db_connection)
            .await?
            .into_iter()
            .map(|model| model.monk_id)
            .collect();

        let monks = self
            .monk_query_service
            .get_by_ids(&monk_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error)
            })?;

        MonasteryMapper::to_domain_entity(model, monks)
            .map_err(|error| DomainError::from(MonasteryErrorKind::Restore).with_cause(error))
    }

    /// Gets the [`Monasteries`][Vec<Monastery>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Monastery>, DomainError<MonasteryErrorKind>> {
        let models = self.repository.get_by_ids(ids, db_connection).await?;

        let ids: Vec<i32> = models.iter().map(|model| model.id).collect();

        let all_monastery_monks = self
            .repository
            .get_monastery_monks_by_monastery_ids(&ids, db_connection)
            .await?;

        let monk_ids: Vec<i32> = all_monastery_monks
            .iter()
            .map(|model| model.monk_id)
            .collect();

        let all_monks = self
            .monk_query_service
            .get_by_ids(&monk_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(MonasteryErrorKind::GetAllMonks).with_cause(error)
            })?;

        models
            .into_iter()
            .map(|monastery_model| {
                let monk_ids: Vec<i32> = all_monastery_monks
                    .iter()
                    .filter(|model| model.monastery_id == monastery_model.id)
                    .map(|model| &model.monk_id)
                    .cloned()
                    .collect();

                let monks = all_monks
                    .iter()
                    .filter(|monk| monk.id().is_ok_and(|monk_id| monk_ids.contains(&monk_id)))
                    .cloned()
                    .collect();

                MonasteryMapper::to_domain_entity(monastery_model, monks).map_err(|error| {
                    DomainError::from(MonasteryErrorKind::Restore).with_cause(error)
                })
            })
            .collect::<Result<Vec<Monastery>, DomainError<MonasteryErrorKind>>>()
    }
}

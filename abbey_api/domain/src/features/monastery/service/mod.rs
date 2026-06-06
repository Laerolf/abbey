use entity::monastery_monks;
use sea_orm::ConnectionTrait;

use crate::{
    features::{
        actor::{
            domain::monk::Monk,
            forms::{MonasteryMonkCreationForm, MonkCreationForm},
            mapper::MonasteryMonkMapper,
            service::{MonkCommandService, MonkQueryService},
        },
        monastery::{
            domain::Monastery, error::MonasteryErrorKind, mapper::MonasteryMapper,
            repository::MonasteryRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// The default amount of Monks in a Monastery.
const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

/// Represents a command service for [`Monasteries`][Monastery].
#[derive(Clone)]
pub struct MonasteryCommandService {
    repository: MonasteryRepository,
    monastery_query_service: MonasteryQueryService,
    monk_command_service: MonkCommandService,
}

impl MonasteryCommandService {
    /// Creates a new [`MonasteryCommandService`].
    pub fn new(
        repository: MonasteryRepository,
        monastery_query_service: MonasteryQueryService,
        monk_command_service: MonkCommandService,
    ) -> Self {
        Self {
            repository,
            monastery_query_service,
            monk_command_service,
        }
    }

    /// Assigns [Monks][`Vec<Monk>`] to a [``Monastery`].
    async fn assign_monks<C: ConnectionTrait>(
        &self,
        forms: Vec<MonasteryMonkCreationForm>,
        db_connection: &C,
    ) -> Result<Vec<monastery_monks::Model>, DomainError<MonasteryErrorKind>> {
        let models: Vec<monastery_monks::ActiveModel> = forms
            .into_iter()
            .map(MonasteryMonkMapper::to_new_active_model)
            .collect();

        self.repository.assign_monks(models, db_connection).await
    }

    /// Creates a new [`Monastery`].
    pub async fn create<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Monastery, DomainError<MonasteryErrorKind>> {
        let skill_names = vec!["cooking".to_string(), "brewing".to_string()];

        let model_plan = MonasteryMapper::to_new_active_model();

        let model = self
            .repository
            .create(model_plan, db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))?;

        let monk_creation_forms = (0..DEFAULT_AMOUNT_OF_MONKS)
            .map(|_| MonkCreationForm::new("Maurits", skill_names.clone()))
            .collect();

        let monks: Vec<Monk> = self
            .monk_command_service
            .create_many(monk_creation_forms, db_connection)
            .await
            .map_err(|error| DomainError::from(MonasteryErrorKind::Creation).with_cause(error))?;

        let monk_assignments: Vec<MonasteryMonkCreationForm> = monks
            .iter()
            .map(|monk| {
                Ok(MonasteryMonkCreationForm::new(
                    model.id,
                    monk.id().map_err(|error| {
                        DomainError::from(MonasteryErrorKind::AssignMonks).with_cause(error)
                    })?,
                ))
            })
            .collect::<Result<Vec<MonasteryMonkCreationForm>, DomainError<MonasteryErrorKind>>>()?;

        self.assign_monks(monk_assignments, db_connection).await?;

        self.monastery_query_service
            .get_by_id(&model.id, db_connection)
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

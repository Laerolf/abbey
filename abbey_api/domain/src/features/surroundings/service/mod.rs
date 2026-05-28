use sea_orm::{ConnectionTrait, DatabaseTransaction};
use time::Duration;

use crate::{
    features::{
        output::{
            domain::resource::{Category, Resource},
            service::ResourceService,
        },
        process::{
            domain::cyclic_process::CyclicProcess, service::cyclic_process::CyclicProcessService,
        },
        source::{
            domain::Source,
            error::SourceErrorKind,
            service::{SourceQueryService, SourceService},
        },
        surroundings::{
            domain::Surroundings, error::SurroundingsErrorKind, forms::SurroundingsCreationForm,
            mapper::SurroundingsMapper, repository::SurroundingsRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service handling the [`Surroundings`] topic.
#[derive(Clone)]
pub struct SurroundingsService {
    repository: SurroundingsRepository,
    source_service: SourceService,
    cyclic_process_service: CyclicProcessService,
    resource_service: ResourceService,
}

impl SurroundingsService {
    /// Creates a new [`SurroundingsService`].
    pub fn new(
        repository: SurroundingsRepository,
        source_service: SourceService,
        cyclic_process_service: CyclicProcessService,
        resource_service: ResourceService,
    ) -> Self {
        Self {
            repository,
            source_service,
            cyclic_process_service,
            resource_service,
        }
    }

    /// Creates the [Sources][Vec<Source>] for a new [`Surroundings`].
    async fn create_sources(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Source>, DomainError<SourceErrorKind>> {
        let beach_resources: Vec<Resource> = vec![
            self.resource_service
                .find_by_name_or_create("sand", Category::Material, db_transaction)
                .await
                .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?,
            self.resource_service
                .find_by_name_or_create("seaweed", Category::Material, db_transaction)
                .await
                .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?,
        ];

        let beach_process: CyclicProcess = self
            .cyclic_process_service
            .create(beach_resources, Duration::minutes(1), db_transaction)
            .await
            .map_err(|error| DomainError::from(SourceErrorKind::Creation).with_cause(error))?;

        let beach: Source = self
            .source_service
            .create_source("the_beach", beach_process, db_transaction)
            .await?;

        Ok(vec![beach])
    }

    /// Creates a new [`Surroundings`].
    pub async fn create_surroundings(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Surroundings, DomainError<SurroundingsErrorKind>> {
        let source_ids: Vec<i32> = self
            .create_sources(db_transaction)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))?
            .iter()
            .map(|source| source.id().unwrap())
            .collect();

        let creation_form = SurroundingsCreationForm::new(source_ids);

        self.repository.create(creation_form, db_transaction).await
    }
}

/// Represents a query service for [`Surroundings`].
#[derive(Clone)]
pub struct SurroundingsQueryService {
    repository: SurroundingsRepository,
    source_query_service: SourceQueryService,
}

impl SurroundingsQueryService {
    /// Creates a new [`SurroundingsQueryService`].
    pub fn new(
        repository: SurroundingsRepository,
        source_query_service: SourceQueryService,
    ) -> Self {
        Self {
            repository,
            source_query_service,
        }
    }

    /// Gets a [`Surroundings`] for the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Surroundings, DomainError<SurroundingsErrorKind>> {
        let model = self
            .repository
            .find_by_id(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(SurroundingsErrorKind::GetById))?;

        let source_ids: Vec<i32> = self
            .repository
            .get_all_sources_by_surroundings_id(&model.id, db_connection)
            .await?
            .iter()
            .map(|source_model| source_model.id)
            .collect();

        let sources = self
            .source_query_service
            .get_by_ids(&source_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })?;

        SurroundingsMapper::to_domain_entity(model, sources)
            .map_err(|error| DomainError::from(SurroundingsErrorKind::GetById).with_cause(error))
    }

    /// Gets a all the [`Surroundings`][Vec<Surroundings>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Surroundings>, DomainError<SurroundingsErrorKind>> {
        let models = self.repository.get_by_ids(ids, db_connection).await?;

        let ids: Vec<i32> = models.iter().map(|model| model.id).collect();

        let surrounding_sources = self
            .repository
            .get_all_sources_by_surroundings_ids(&ids, db_connection)
            .await?;

        let all_source_ids: Vec<i32> = surrounding_sources
            .iter()
            .map(|source_model| source_model.id)
            .collect();

        let all_sources = self
            .source_query_service
            .get_by_ids(&all_source_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::GetAllSources).with_cause(error)
            })?;

        models
            .into_iter()
            .map(|surroundings_model| {
                let source_ids: Vec<i32> = surrounding_sources
                    .iter()
                    .filter(|model| model.surroundings_id == surroundings_model.id)
                    .map(|model| model.source_id)
                    .collect();

                let sources = all_sources
                    .iter()
                    .filter(|source| {
                        source
                            .id()
                            .is_ok_and(|source_id| source_ids.contains(&source_id))
                    })
                    .cloned()
                    .collect();

                SurroundingsMapper::to_domain_entity(surroundings_model, sources).map_err(|error| {
                    DomainError::from(SurroundingsErrorKind::GetById).with_cause(error)
                })
            })
            .collect::<Result<Vec<Surroundings>, DomainError<SurroundingsErrorKind>>>()
    }
}

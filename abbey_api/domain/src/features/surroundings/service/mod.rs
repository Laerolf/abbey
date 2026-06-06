use entity::surroundings_sources;
use sea_orm::ConnectionTrait;

use crate::{
    features::{
        output::{
            error::ResourceErrorKind, forms::ResourceBlueprint, service::ResourceCommandService,
        },
        process::{
            error::ProcessErrorKind, forms::cyclic_process::CyclicProcessBlueprint,
            service::cyclic_process::CyclicProcessCommandService,
        },
        source::{
            domain::Source,
            error::SourceErrorKind,
            forms::SourceBlueprint,
            service::{SourceCommandService, SourceQueryService},
        },
        surroundings::{
            domain::Surroundings,
            error::SurroundingsErrorKind,
            forms::{
                SurroundingSourceAssignmentForm, SurroundingsBlueprint,
                SurroundingsCyclicProcessSourceBlueprint, SurroundingsSourceBlueprint,
            },
            mapper::{SurroundingsMapper, SurroundingsSourceMapper},
            repository::SurroundingsRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for [`Surroundings`].
#[derive(Clone)]
pub struct SurroundingsCommandService {
    repository: SurroundingsRepository,
    surroundings_query_service: SurroundingsQueryService,
    source_command_service: SourceCommandService,
    resource_command_service: ResourceCommandService,
    cyclic_process_command_service: CyclicProcessCommandService,
}

impl SurroundingsCommandService {
    /// Creates a new [`SurroundingsCommandService`].
    pub fn new(
        repository: SurroundingsRepository,
        surroundings_query_service: SurroundingsQueryService,
        source_command_service: SourceCommandService,
        resource_command_service: ResourceCommandService,
        cyclic_process_command_service: CyclicProcessCommandService,
    ) -> Self {
        Self {
            repository,
            surroundings_query_service,
            source_command_service,
            resource_command_service,
            cyclic_process_command_service,
        }
    }

    /// Creates the [Sources][Vec<Source>] for a [´Surroundings´].
    async fn create_sources<C: ConnectionTrait>(
        &self,
        blueprints: Vec<SurroundingsSourceBlueprint>,
        db_connection: &C,
    ) -> Result<Vec<Source>, DomainError<SurroundingsErrorKind>> {
        let all_source_cyclic_process_blueprints: Vec<SurroundingsCyclicProcessSourceBlueprint> =
            blueprints
                .clone()
                .into_iter()
                .map(|blueprint| blueprint.cyclic_process_blueprint)
                .collect();

        let all_output_resource_blueprints: Vec<ResourceBlueprint> =
            all_source_cyclic_process_blueprints
                .clone()
                .into_iter()
                .flat_map(|blueprint| blueprint.resource_blueprints)
                .collect();

        let all_output_resources = self
            .resource_command_service
            .get_or_create_many(all_output_resource_blueprints, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::Creation).with_cause(error)
            })?;

        let cyclic_process_blueprints: Vec<CyclicProcessBlueprint> =
            all_source_cyclic_process_blueprints
                .into_iter()
                .map(|cyclic_process_blueprint| {
                    let output_resource_ids: Vec<i32> = all_output_resources
                        .iter()
                        .filter(|resource| {
                            cyclic_process_blueprint
                                .resource_blueprints
                                .iter()
                                .any(|blueprint| blueprint.name == *resource.name())
                        })
                        .map(|resource| resource.id())
                        .collect::<Result<Vec<i32>, DomainError<ResourceErrorKind>>>()
                        .map_err(|error| {
                            DomainError::from(ProcessErrorKind::Creation).with_cause(error)
                        })?;

                    CyclicProcessBlueprint::new(
                        output_resource_ids,
                        cyclic_process_blueprint.cyclic_process_cycle_interval,
                    )
                })
                .collect::<Result<Vec<CyclicProcessBlueprint>, DomainError<ProcessErrorKind>>>()
                .map_err(|error| {
                    DomainError::from(SurroundingsErrorKind::Creation).with_cause(error)
                })?;

        let cyclic_processes = self
            .cyclic_process_command_service
            .create_many(cyclic_process_blueprints, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::Creation).with_cause(error)
            })?;

        let source_blueprints: Vec<SourceBlueprint> = blueprints
            .into_iter()
            .zip(cyclic_processes)
            .map(|(blueprint, cyclic_process)| {
                let cyclic_process_id = cyclic_process.id().map_err(|error| {
                    DomainError::from(SourceErrorKind::Creation).with_cause(error)
                })?;

                Ok(SourceBlueprint::new(
                    blueprint.source_name,
                    cyclic_process_id,
                ))
            })
            .collect::<Result<Vec<SourceBlueprint>, DomainError<SourceErrorKind>>>()
            .map_err(|error| {
                DomainError::from(SurroundingsErrorKind::Creation).with_cause(error)
            })?;

        self.source_command_service
            .create_many(source_blueprints, db_connection)
            .await
            .map_err(|error| DomainError::from(SurroundingsErrorKind::Creation).with_cause(error))
    }

    /// Creates a new [`Surroundings`].
    pub async fn create<C: ConnectionTrait>(
        &self,
        blueprint: SurroundingsBlueprint,
        db_connection: &C,
    ) -> Result<Surroundings, DomainError<SurroundingsErrorKind>> {
        let sources = self
            .create_sources(blueprint.source_blueprints, db_connection)
            .await?;

        let source_ids: Vec<i32> = sources.iter().map(|source| source.id().unwrap()).collect();

        let model = self
            .repository
            .create(SurroundingsMapper::to_new_active_model(), db_connection)
            .await?;

        let surroundings_sources: Vec<surroundings_sources::ActiveModel> = source_ids
            .into_iter()
            .map(|source_id| {
                SurroundingsSourceMapper::to_new_active_model(SurroundingSourceAssignmentForm::new(
                    model.id, source_id,
                ))
            })
            .collect();

        self.repository
            .assign_sources_to_surroundings(surroundings_sources, db_connection)
            .await?;

        self.surroundings_query_service
            .get_by_id(&model.id, db_connection)
            .await
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

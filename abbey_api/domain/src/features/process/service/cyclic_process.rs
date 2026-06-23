use entity::{cyclic_process_resources, cyclic_processes};
use sea_orm::ConnectionTrait;
use time::OffsetDateTime;

use crate::{
    features::{
        actor::{domain::ActorKind, service::ActorQueryService},
        output::{domain::resource::Resource, service::ResourceQueryService},
        process::{
            domain::{Process, cyclic_process::CyclicProcess},
            error::ProcessErrorKind,
            forms::cyclic_process::{
                CyclicProcessBlueprint, CyclicProcessOutputResourceAssignmentForm,
            },
            mapper::cyclic_process::{CyclicProcessMapper, CyclicProcessResourceMapper},
            repository::cyclic_process::CyclicProcessRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for [`CyclicProcesses`][CyclicProcess].
#[derive(Clone)]
pub struct CyclicProcessCommandService {
    repository: CyclicProcessRepository,
    cyclic_process_query_service: CyclicProcessQueryService,
}

impl CyclicProcessCommandService {
    /// Creates a new [`CyclicProcessCommandService`].
    pub fn new(
        repository: CyclicProcessRepository,
        cyclic_process_query_service: CyclicProcessQueryService,
    ) -> Self {
        Self {
            repository,
            cyclic_process_query_service,
        }
    }

    /// Creates new [`CyclicProcesses`][Vec<CyclicProcess>].
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        blueprints: Vec<CyclicProcessBlueprint>,
        db_connection: &C,
    ) -> Result<Vec<CyclicProcess>, DomainError<ProcessErrorKind>> {
        let active_models: Vec<cyclic_processes::ActiveModel> = blueprints
            .clone()
            .into_iter()
            .map(CyclicProcessMapper::to_new_active_model)
            .collect::<Result<Vec<cyclic_processes::ActiveModel>, DomainError<ProcessErrorKind>>>(
            )?;

        let models = self
            .repository
            .create_many(active_models, db_connection)
            .await?;

        let output_resource_assignments: Vec<cyclic_process_resources::ActiveModel> = blueprints
            .iter()
            .zip(models.iter())
            .flat_map(|(blueprint, model)| {
                blueprint
                    .output_resources_ids
                    .iter()
                    .map(|resource_id| {
                        CyclicProcessResourceMapper::to_new_active_model(
                            CyclicProcessOutputResourceAssignmentForm::new(model.id, *resource_id),
                        )
                    })
                    .collect::<Vec<cyclic_process_resources::ActiveModel>>()
            })
            .collect();

        self.repository
            .assign_resources_to_cyclic_process(output_resource_assignments, db_connection)
            .await?;

        let ids: Vec<i32> = models.iter().map(|model| model.id).collect();

        self.cyclic_process_query_service
            .get_by_ids(&ids, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Updates a [`CyclicProcess`].
    pub async fn update<C: ConnectionTrait>(
        &self,
        cyclic_process: CyclicProcess,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let active_model = CyclicProcessMapper::to_update_active_model(cyclic_process);

        let updated_model = self.repository.update(active_model, db_connection).await?;

        self.cyclic_process_query_service
            .get_by_id(&updated_model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Update).with_cause(error))
    }

    /// Starts a [`CyclicProcess`].
    pub async fn start<C: ConnectionTrait>(
        &self,
        process_id: &i32,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let mut model = self
            .cyclic_process_query_service
            .get_by_id(process_id, db_connection)
            .await
            .map_err(|_error| DomainError::from(ProcessErrorKind::NotFound))?;

        model.start(OffsetDateTime::now_utc()).map_err(|error| {
            DomainError::from(ProcessErrorKind::Start)
                .with_cause(error)
                .with_context("process_id", process_id.to_string())
        })?;

        let active_model = CyclicProcessMapper::to_update_active_model(model);

        let updated_model = self
            .repository
            .update(active_model, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::Start)
                    .with_cause(error)
                    .with_context("process_id", process_id.to_string())
            })?;

        self.cyclic_process_query_service
            .get_by_id(&updated_model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Start).with_cause(error))
    }

    /// Starts a [`CyclicProcess`].
    pub async fn pause<C: ConnectionTrait>(
        &self,
        process_id: &i32,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let mut model = self
            .cyclic_process_query_service
            .get_by_id(process_id, db_connection)
            .await
            .map_err(|_error| DomainError::from(ProcessErrorKind::NotFound))?;

        model.pause(OffsetDateTime::now_utc()).map_err(|error| {
            DomainError::from(ProcessErrorKind::Pause)
                .with_cause(error)
                .with_context("process_id", process_id.to_string())
        })?;

        let active_model = CyclicProcessMapper::to_update_active_model(model);

        let updated_model = self
            .repository
            .update(active_model, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::Pause)
                    .with_cause(error)
                    .with_context("process_id", process_id.to_string())
            })?;

        self.cyclic_process_query_service
            .get_by_id(&updated_model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Pause).with_cause(error))
    }
}

/// Represents a query service for [`CyclicProcesses`][CyclicProcess].
#[derive(Clone)]
pub struct CyclicProcessQueryService {
    repository: CyclicProcessRepository,
    resource_query_service: ResourceQueryService,
    actor_query_service: ActorQueryService,
}

impl CyclicProcessQueryService {
    /// Creates a new [`CyclicProcessQueryService`].
    pub fn new(
        repository: CyclicProcessRepository,
        resource_query_service: ResourceQueryService,
        actor_query_service: ActorQueryService,
    ) -> Self {
        Self {
            repository,
            resource_query_service,
            actor_query_service,
        }
    }

    /// Finds a [`CyclicProcess`] with the provided ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<CyclicProcess>, DomainError<ProcessErrorKind>> {
        let Some(model) = self.repository.find_by_id(id, db_connection).await? else {
            return Ok(None);
        };

        let output_resource_assignments = self
            .repository
            .get_resource_assignments_by_cyclic_process_id(&model.id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        let output_resource_ids: Vec<i32> = output_resource_assignments
            .iter()
            .map(|assignment| assignment.resource_id)
            .collect();

        let output_resources: Vec<Resource> = self
            .resource_query_service
            .get_by_ids(&output_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        Ok(Some(CyclicProcessMapper::to_domain_entity(
            model,
            output_resources,
            Vec::new(),
        )?))
    }

    /// Gets a [`CyclicProcess`] with the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let model = self
            .repository
            .find_by_id(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::GetById))?;

        let output_resource_assignments = self
            .repository
            .get_resource_assignments_by_cyclic_process_id(&model.id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        let output_resource_ids: Vec<i32> = output_resource_assignments
            .iter()
            .map(|assignment| assignment.resource_id)
            .collect();

        let output_resources: Vec<Resource> = self
            .resource_query_service
            .get_by_ids(&output_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        let assigned_actors: Vec<ActorKind> = self
            .actor_query_service
            .get_process_actors_by_process_id(&model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?
            .into_iter()
            .map(|link| link.actor().clone())
            .collect();

        CyclicProcessMapper::to_domain_entity(model, output_resources, assigned_actors)
    }

    /// Gets the [`CyclicProcesses`][Vec<CyclicProcess>] with the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<CyclicProcess>, DomainError<ProcessErrorKind>> {
        let models = self
            .repository
            .get_by_ids(ids, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetByIds).with_cause(error))?;

        let output_resource_assignments = self
            .repository
            .get_resource_assignments_by_cyclic_process_ids(ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        let output_resource_ids: Vec<i32> = output_resource_assignments
            .iter()
            .map(|assignment| assignment.resource_id)
            .collect();

        let output_resources: Vec<Resource> = self
            .resource_query_service
            .get_by_ids(&output_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        let all_assigned_actor_links = self
            .actor_query_service
            .get_process_actors_by_process_ids(ids, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindActors).with_cause(error))?;

        models
            .into_iter()
            .map(|cyclic_process_model| {
                let output_resource_ids: Vec<i32> = output_resource_assignments
                    .iter()
                    .filter(|assignment| assignment.cyclic_process_id == cyclic_process_model.id)
                    .clone()
                    .map(|assignment| assignment.resource_id)
                    .collect();

                let output_resources: Vec<Resource> = output_resources
                    .clone()
                    .into_iter()
                    .filter(|resource| output_resource_ids.contains(&resource.id().unwrap()))
                    .collect();

                let assigned_actors: Vec<ActorKind> = all_assigned_actor_links
                    .iter()
                    .filter(|link| *link.process_id() == cyclic_process_model.id)
                    .map(|link| link.actor())
                    .cloned()
                    .collect();

                CyclicProcessMapper::to_domain_entity(
                    cyclic_process_model,
                    output_resources,
                    assigned_actors,
                )
            })
            .collect::<Result<Vec<CyclicProcess>, DomainError<ProcessErrorKind>>>()
    }
}

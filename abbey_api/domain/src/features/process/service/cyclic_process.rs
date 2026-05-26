use sea_orm::{ConnectionTrait, DatabaseTransaction};
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::{domain::ActorKind, service::ActorQueryService},
        output::{domain::resource::Resource, service::ResourceQueryService},
        process::{
            domain::{Process, ProcessKind, cyclic_process::CyclicProcess},
            error::ProcessErrorKind,
            forms::cyclic_process::CyclicProcessCreationForm,
            mapper::cyclic_process::CyclicProcessMapper,
            repository::cyclic_process::CyclicProcessRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service handling the [`CyclicProcess`] topic.
#[derive(Clone)]
pub struct CyclicProcessService {
    repository: CyclicProcessRepository,
}

impl CyclicProcessService {
    /// Creates a new [`CyclicProcessService`].
    pub fn new(repository: CyclicProcessRepository) -> Self {
        Self { repository }
    }

    /// Finds a [`CyclicProcess`] and all its related entities by its ID.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<CyclicProcess>, DomainError<ProcessErrorKind>> {
        self.repository
            .find_by_id_with_relations(id, db_connection)
            .await
    }

    /// Creates a new [`CyclicProcess`].
    pub async fn create(
        &self,
        output_resources: Vec<Resource>,
        cycle_interval: Duration,
        db_transaction: &DatabaseTransaction,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        let creation_form = CyclicProcessCreationForm::new(
            output_resources
                .iter()
                .map(|resource| resource.id().unwrap())
                .collect(),
            cycle_interval,
        );

        self.repository.create(creation_form, db_transaction).await
    }

    /// Starts a [`CyclicProcess`].
    pub async fn start_by_id_in_game(
        &self,
        process_id: &i32,
        game_id: &i32,
        db_transaction: &DatabaseTransaction,
    ) -> Result<ProcessKind, DomainError<ProcessErrorKind>> {
        let mut process = self
            .repository
            .get_by_id_for_game_with_relations(process_id, game_id, db_transaction)
            .await?;

        process.start(OffsetDateTime::now_utc()).map_err(|error| {
            DomainError::from(ProcessErrorKind::Start)
                .with_cause(error)
                .with_context("process_id", process_id.to_string())
        })?;

        process = self
            .repository
            .update(process, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::Start)
                    .with_cause(error)
                    .with_context("process_id", process_id.to_string())
            })?;

        Ok(ProcessKind::CyclicProcess(process))
    }

    /// Pauses a [`CyclicProcess`].
    pub async fn pause_by_id_in_game(
        &self,
        process_id: &i32,
        game_id: &i32,
        db_transaction: &DatabaseTransaction,
    ) -> Result<ProcessKind, DomainError<ProcessErrorKind>> {
        let mut process = self
            .repository
            .get_by_id_for_game_with_relations(process_id, game_id, db_transaction)
            .await?;

        process.pause(OffsetDateTime::now_utc()).map_err(|error| {
            DomainError::from(ProcessErrorKind::Start)
                .with_cause(error)
                .with_context("process_id", process_id.to_string())
        })?;

        process = self
            .repository
            .update(process, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::Start)
                    .with_cause(error)
                    .with_context("process_id", process_id.to_string())
            })?;

        Ok(ProcessKind::CyclicProcess(process))
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

    /// Get a [`CyclicProcess`] for the provided ID.
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
            .get_by_ids(output_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        CyclicProcessMapper::to_domain_entity(model, output_resources, Vec::new())
    }

    /// Gets the [`CyclicProcesses`][Vec<CyclicProcess>] for the provided IDs.
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
            .get_by_ids(output_resource_ids, db_connection)
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

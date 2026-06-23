use entity::{task_input_resources, task_output_resources};
use sea_orm::ConnectionTrait;
use time::OffsetDateTime;

use crate::{
    features::{
        output::service::ResourceQueryService,
        process::{
            domain::{Process, task::Task},
            error::ProcessErrorKind,
            forms::task::{
                TaskBlueprint, TaskInputResourceAssignmentForm, TaskOutputResourceAssignmentForm,
            },
            mapper::task::{TaskInputResourceMapper, TaskMapper, TaskOutputResourceMapper},
            repository::task::TaskRepository,
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for [`Tasks`][Task].
#[derive(Clone)]
pub struct TaskCommandService {
    repository: TaskRepository,
    task_query_service: TaskQueryService,
}

impl TaskCommandService {
    /// Creates a new [`TaskCommandService`].
    pub fn new(repository: TaskRepository, task_query_service: TaskQueryService) -> Self {
        Self {
            repository,
            task_query_service,
        }
    }

    /// Creates a new [`Task`].
    pub async fn create<C: ConnectionTrait>(
        &self,
        blueprint: TaskBlueprint,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let active_model = TaskMapper::to_new_active_model(blueprint.clone());

        let model = self.repository.create(active_model, db_connection).await?;

        let input_resource_active_models: Vec<task_input_resources::ActiveModel> = blueprint
            .clone()
            .input_resources_ids
            .iter()
            .map(|resource_id| {
                TaskInputResourceMapper::to_new_active_model(TaskInputResourceAssignmentForm::new(
                    model.id,
                    *resource_id,
                ))
            })
            .collect();

        self.repository
            .assign_input_resources_to_task(input_resource_active_models, db_connection)
            .await?;

        let output_resource_active_models: Vec<task_output_resources::ActiveModel> = blueprint
            .clone()
            .output_resources_ids
            .iter()
            .map(|resource_id| {
                TaskOutputResourceMapper::to_new_active_model(
                    TaskOutputResourceAssignmentForm::new(model.id, *resource_id),
                )
            })
            .collect();

        self.repository
            .assign_output_resources_to_task(output_resource_active_models, db_connection)
            .await?;

        self.task_query_service
            .get_by_id(&model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Updates a [`Task`].
    pub async fn update<C: ConnectionTrait>(
        &self,
        task: Task,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let active_model = TaskMapper::to_update_active_model(task);

        let updated_model = self.repository.update(active_model, db_connection).await?;

        self.task_query_service
            .get_by_id(&updated_model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Update).with_cause(error))
    }

    /// Starts a [`Task`].
    pub async fn start<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let mut model = self.task_query_service.get_by_id(id, db_connection).await?;

        model.start(OffsetDateTime::now_utc()).map_err(|error| {
            DomainError::from(ProcessErrorKind::Start)
                .with_cause(error)
                .with_context("process_id", id.to_string())
        })?;

        let active_model = TaskMapper::to_update_active_model(model);

        let updated_model = self
            .repository
            .update(active_model, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Start).with_cause(error))?;

        self.task_query_service
            .get_by_id(&updated_model.id, db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Start).with_cause(error))
    }
}

/// Represents a query service for [`Tasks`][Task].
#[derive(Clone)]
pub struct TaskQueryService {
    repository: TaskRepository,
    resource_query_service: ResourceQueryService,
}

impl TaskQueryService {
    /// Creates a new [`TaskQueryService`].
    pub fn new(repository: TaskRepository, resource_query_service: ResourceQueryService) -> Self {
        Self {
            repository,
            resource_query_service,
        }
    }

    /// Gets a [`Task`] with the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let model = self
            .repository
            .find_by_id(id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ProcessErrorKind::GetById))?;

        let input_resource_ids: Vec<i32> = self
            .repository
            .get_input_resource_assignments_by_task_id(&model.id, db_connection)
            .await?
            .iter()
            .map(|resource| resource.id)
            .collect();

        let input_resources = self
            .resource_query_service
            .get_by_ids(&input_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        let output_resource_ids: Vec<i32> = self
            .repository
            .get_output_resource_assignments_by_task_id(&model.id, db_connection)
            .await?
            .iter()
            .map(|resource| resource.id)
            .collect();

        let output_resources = self
            .resource_query_service
            .get_by_ids(&output_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        TaskMapper::to_domain_entity(model, input_resources, output_resources, Vec::new())
    }

    /// Gets a [`Tasks`][Vec<Task>] with the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Task>, DomainError<ProcessErrorKind>> {
        let models = self.repository.get_by_ids(ids, db_connection).await?;

        let ids: Vec<i32> = models.iter().map(|model| model.id).collect();

        let all_input_resource_assignments = self
            .repository
            .get_input_resource_assignments_by_task_ids(&ids, db_connection)
            .await?;

        let all_input_resource_ids: Vec<i32> = all_input_resource_assignments
            .iter()
            .map(|resource| resource.id)
            .collect();

        let all_input_resources = self
            .resource_query_service
            .get_by_ids(&all_input_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        let all_output_resource_assignments = self
            .repository
            .get_output_resource_assignments_by_task_ids(&ids, db_connection)
            .await?;

        let all_output_resource_ids: Vec<i32> = all_output_resource_assignments
            .iter()
            .map(|resource| resource.id)
            .collect();

        let all_output_resources = self
            .resource_query_service
            .get_by_ids(&all_output_resource_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error)
            })?;

        models
            .into_iter()
            .map(|task_model| {
                let input_resource_ids: Vec<i32> = all_input_resource_assignments
                    .iter()
                    .filter(|assignment| assignment.task_id == task_model.id)
                    .map(|assignment| assignment.resource_id)
                    .collect();

                let input_resources = all_input_resources
                    .iter()
                    .filter(|resource| {
                        resource
                            .id()
                            .is_ok_and(|id| input_resource_ids.contains(&id))
                    })
                    .cloned()
                    .collect();

                let output_resource_ids: Vec<i32> = all_output_resource_assignments
                    .iter()
                    .filter(|assignment| assignment.task_id == task_model.id)
                    .map(|assignment| assignment.resource_id)
                    .collect();

                let output_resources = all_output_resources
                    .iter()
                    .filter(|resource| {
                        resource
                            .id()
                            .is_ok_and(|id| output_resource_ids.contains(&id))
                    })
                    .cloned()
                    .collect();

                TaskMapper::to_domain_entity(
                    task_model,
                    input_resources,
                    output_resources,
                    Vec::new(),
                )
            })
            .collect::<Result<Vec<Task>, DomainError<ProcessErrorKind>>>()
    }
}

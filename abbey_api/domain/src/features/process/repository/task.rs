use entity::{task_input_resources, task_output_resources, tasks};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{features::process::error::ProcessErrorKind, shared::error::DomainError};

/// Represents an element that handles all [Task] database topics.
#[derive(Default, Clone)]
pub struct TaskRepository;

impl TaskRepository {
    /// Creates a [`Task`][tasks::Model] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: tasks::ActiveModel,
        db_connection: &C,
    ) -> Result<tasks::Model, DomainError<ProcessErrorKind>> {
        tasks::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Assigns input Resources to a [`Task`].
    pub async fn assign_input_resources_to_task<C: ConnectionTrait>(
        &self,
        models: Vec<task_input_resources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<task_input_resources::Model>, DomainError<ProcessErrorKind>> {
        task_input_resources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Assigns output Resources to a [`Task`].
    pub async fn assign_output_resources_to_task<C: ConnectionTrait>(
        &self,
        models: Vec<task_output_resources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<task_output_resources::Model>, DomainError<ProcessErrorKind>> {
        task_output_resources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Finds a [`Task`][tasks::Model] with the provided ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<tasks::Model>, DomainError<ProcessErrorKind>> {
        tasks::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindById).with_cause(error))
    }

    /// Gets all [`Tasks`][Vec<tasks::Model>] with the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<tasks::Model>, DomainError<ProcessErrorKind>> {
        tasks::Entity::find()
            .filter(tasks::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetByIds).with_cause(error))
    }

    /// Gets the input Resource assignments for a [`Task`] with the provided ID.
    pub async fn get_input_resource_assignments_by_task_id<C: ConnectionTrait>(
        &self,
        task_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<task_input_resources::Model>, DomainError<ProcessErrorKind>> {
        task_input_resources::Entity::find()
            .filter(task_input_resources::Column::TaskId.eq(*task_id))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error))
    }

    /// Gets the input Resource assignments for [`Tasks`][Vec<Task>] with the provided IDs.
    pub async fn get_input_resource_assignments_by_task_ids<C: ConnectionTrait>(
        &self,
        task_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<task_input_resources::Model>, DomainError<ProcessErrorKind>> {
        task_input_resources::Entity::find()
            .filter(task_input_resources::Column::TaskId.is_in(task_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error))
    }

    /// Gets the output Resource assignments for a [`Task`] with the provided ID.
    pub async fn get_output_resource_assignments_by_task_id<C: ConnectionTrait>(
        &self,
        task_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<task_output_resources::Model>, DomainError<ProcessErrorKind>> {
        task_output_resources::Entity::find()
            .filter(task_output_resources::Column::TaskId.eq(*task_id))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error))
    }

    /// Gets the output Resource assignments for [`Tasks`][Vec<Task>] with the provided IDs.
    pub async fn get_output_resource_assignments_by_task_ids<C: ConnectionTrait>(
        &self,
        task_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<task_output_resources::Model>, DomainError<ProcessErrorKind>> {
        task_output_resources::Entity::find()
            .filter(task_output_resources::Column::TaskId.is_in(task_ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error))
    }

    /// Updates a [`Task`].
    pub async fn update<C: ConnectionTrait>(
        &self,
        model: tasks::ActiveModel,
        db_connection: &C,
    ) -> Result<tasks::Model, DomainError<ProcessErrorKind>> {
        tasks::Entity::update(model)
            .exec(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Update).with_cause(error))
    }
}

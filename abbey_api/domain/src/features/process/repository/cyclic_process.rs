use entity::{cyclic_process_resources, cyclic_processes};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::warn;

use crate::{features::process::error::ProcessErrorKind, shared::error::DomainError};

/// Represents an element that handles all [CyclicProcess] database topics.
#[derive(Default, Clone)]
pub struct CyclicProcessRepository;

impl CyclicProcessRepository {
    /// Finds a [`CyclicProcess`][cyclic_processes::Model] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<cyclic_processes::Model>, DomainError<ProcessErrorKind>> {
        cyclic_processes::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::FindById).with_cause(error))
    }

    /// Gets the [`CyclicProcesses`][Vec<cyclic_processes::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<cyclic_processes::Model>, DomainError<ProcessErrorKind>> {
        cyclic_processes::Entity::find()
            .filter(cyclic_processes::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetByIds).with_cause(error))
    }

    /// Gets the [`Resource Assignments`][Vec<cyclic_process_resources::Model>] for the provided CyclicProcess ID.
    pub async fn get_resource_assignments_by_cyclic_process_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Vec<cyclic_process_resources::Model>, DomainError<ProcessErrorKind>> {
        cyclic_process_resources::Entity::find()
            .filter(cyclic_process_resources::Column::CyclicProcessId.eq(*id))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error))
    }

    /// Gets the [`Resource Assignments`][Vec<cyclic_process_resources::Model>] for the provided CyclicProcess IDs.
    pub async fn get_resource_assignments_by_cyclic_process_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<cyclic_process_resources::Model>, DomainError<ProcessErrorKind>> {
        cyclic_process_resources::Entity::find()
            .filter(cyclic_process_resources::Column::CyclicProcessId.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::GetAllResources).with_cause(error))
    }

    /// Creates a [`CyclicProcess`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: cyclic_processes::ActiveModel,
        db_connection: &C,
    ) -> Result<cyclic_processes::Model, DomainError<ProcessErrorKind>> {
        cyclic_processes::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Creates many new [`CyclicProcesses`][Vec<CyclicProcess>] and persists them in the database.
    pub async fn create_many<C: ConnectionTrait>(
        &self,
        models: Vec<cyclic_processes::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<cyclic_processes::Model>, DomainError<ProcessErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        cyclic_processes::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Assigns Resources to a [`CyclicProcess`].
    pub async fn assign_resources_to_cyclic_process<C: ConnectionTrait>(
        &self,
        models: Vec<cyclic_process_resources::ActiveModel>,
        db_connection: &C,
    ) -> Result<Vec<cyclic_process_resources::Model>, DomainError<ProcessErrorKind>> {
        if models.is_empty() {
            warn!("Skipping this insertion because no models were provided.");
            return Ok(vec![]);
        }

        cyclic_process_resources::Entity::insert_many(models)
            .exec_with_returning_many(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Creation).with_cause(error))
    }

    /// Updates a [`CyclicProcess`].
    pub async fn update<C: ConnectionTrait>(
        &self,
        model: cyclic_processes::ActiveModel,
        db_connection: &C,
    ) -> Result<cyclic_processes::Model, DomainError<ProcessErrorKind>> {
        cyclic_processes::Entity::update(model)
            .exec(db_connection)
            .await
            .map_err(|error| DomainError::from(ProcessErrorKind::Update).with_cause(error))
    }
}

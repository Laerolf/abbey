use sea_orm::{ConnectionTrait, DatabaseTransaction};
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        output::domain::resource::Resource,
        process::{
            domain::{Process, ProcessKind, task::Task},
            error::ProcessErrorKind,
            forms::task::TaskCreationForm,
            repository::task::TaskRepository,
        },
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Task`] topic.
#[derive(Clone)]
pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    /// Creates a new [`TaskService`].
    pub fn new(repository: TaskRepository) -> Self {
        Self { repository }
    }

    /// Finds a [`Task`] and all its related entities by its ID and Game ID.
    pub async fn find_by_id_for_game_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Option<Task>, DomainError<ProcessErrorKind>> {
        self.repository
            .find_by_id_for_game_with_relations(id, game_id, db_connection)
            .await
    }

    /// Creates a new [`Task`].
    pub async fn create(
        &self,
        input_resources: Vec<Resource>,
        output_resources: Vec<Resource>,
        duration: Duration,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        let creation_form = TaskCreationForm::new(
            input_resources
                .iter()
                .map(|resource| resource.id().unwrap())
                .collect(),
            output_resources
                .iter()
                .map(|resource| resource.id().unwrap())
                .collect(),
            duration,
        );

        self.repository.create(creation_form, db_transaction).await
    }

    /// Starts a [`Task`].
    pub async fn start_by_id_in_game(
        &self,
        process_id: &i32,
        game_id: &i32,
        db_transaction: &DatabaseTransaction,
    ) -> Result<ProcessKind, DomainError<ProcessErrorKind>> {
        let mut task = self
            .repository
            .get_by_id_for_game_with_relations(process_id, game_id, db_transaction)
            .await?;

        task.start(OffsetDateTime::now_utc()).map_err(|error| {
            DomainError::from(ProcessErrorKind::Start)
                .with_cause(error)
                .with_context("process_id", process_id.to_string())
        })?;

        task = self
            .repository
            .update(task, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(ProcessErrorKind::Start)
                    .with_cause(error)
                    .with_context("process_id", process_id.to_string())
            })?;

        Ok(ProcessKind::Task(task))
    }
}

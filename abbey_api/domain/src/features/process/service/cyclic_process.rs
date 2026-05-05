use sea_orm::{ConnectionTrait, DatabaseTransaction};
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        output::domain::resource::Resource,
        process::{
            domain::{Process, ProcessKind, cyclic_process::CyclicProcess},
            error::ProcessErrorKind,
            forms::cyclic_process::CyclicProcessCreationForm,
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

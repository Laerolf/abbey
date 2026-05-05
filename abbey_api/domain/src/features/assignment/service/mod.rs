use sea_orm::DatabaseTransaction;

use crate::{
    features::{
        actor::{
            domain::ActorKind,
            repository::{ActorRepository, MonkRepository},
        },
        assignment::{
            domain::{ProcessAssignment, process_assignment_factory::ProcessAssignmentFactory},
            error::AssignmentErrorKind,
            forms::ProcessAssignmentForm,
        },
        game::domain::Game,
        player::repository::PlayerRepository,
        process::{
            domain::ProcessKind,
            repository::{cyclic_process::CyclicProcessRepository, task::TaskRepository},
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service for process assignments.
#[derive(Clone)]
pub struct ProcessAssignmentService {
    player_repository: PlayerRepository,
    monk_repository: MonkRepository,
    cyclic_process_repository: CyclicProcessRepository,
    task_repository: TaskRepository,
    actor_repository: ActorRepository,
}

impl ProcessAssignmentService {
    /// Creates a new [`ProcessAssignmentService`].
    pub fn new(
        player_repository: PlayerRepository,
        monk_repository: MonkRepository,
        cyclic_process_repository: CyclicProcessRepository,
        task_repository: TaskRepository,
        actor_repository: ActorRepository,
    ) -> Self {
        Self {
            player_repository,
            monk_repository,
            cyclic_process_repository,
            task_repository,
            actor_repository,
        }
    }

    /// Assigns a [`Actor`][ActorKind] to a [`Process`][ProcessKind].
    async fn assign_process_to_actors(
        &self,
        actors: Vec<ActorKind>,
        process: ProcessKind,
        db_transaction: &DatabaseTransaction,
    ) -> Result<ProcessAssignment, DomainError<AssignmentErrorKind>> {
        let assignment = ProcessAssignmentFactory::assign_process_to_actors(actors, process)?;

        for actor in assignment.actors() {
            match actor {
                ActorKind::Monk(monk) => {
                    self.monk_repository
                        .update(monk.to_owned(), db_transaction)
                        .await
                        .map_err(|error| {
                            DomainError::from(AssignmentErrorKind::ActorAssignment)
                                .with_cause(error)
                        })?;
                }
                ActorKind::Player(player) => {
                    self.player_repository
                        .update(player.to_owned(), db_transaction)
                        .await
                        .map_err(|error| {
                            DomainError::from(AssignmentErrorKind::ActorAssignment)
                                .with_cause(error)
                        })?;
                }
            }
        }

        match assignment.process() {
            ProcessKind::CyclicProcess(cyclic_process) => {
                self.cyclic_process_repository
                    .update(cyclic_process.to_owned(), db_transaction)
                    .await
                    .map_err(|error| {
                        DomainError::from(AssignmentErrorKind::ProcessAssignment).with_cause(error)
                    })?;
            }
            ProcessKind::Task(task) => {
                self.task_repository
                    .update(task.to_owned(), db_transaction)
                    .await
                    .map_err(|error| {
                        DomainError::from(AssignmentErrorKind::ProcessAssignment).with_cause(error)
                    })?;
            }
        }

        Ok(assignment)
    }

    /// Assigns a [`Process`][ProcessKind] based on the provided [ProcessAssignmentForm].
    pub async fn assign_process_to_actors_in_game(
        &self,
        form: ProcessAssignmentForm,
        game: &Game,
        db_transaction: &DatabaseTransaction,
    ) -> Result<ProcessAssignment, DomainError<AssignmentErrorKind>> {
        let found_process = self
            .cyclic_process_repository
            .get_by_id_for_game_with_relations(
                &form.process_id,
                &game.id().unwrap(),
                db_transaction,
            )
            .await
            .map_err(|error| {
                DomainError::from(AssignmentErrorKind::ProcessNotFound).with_cause(error)
            })?;

        let mut found_actors = if form.actor_ids.is_empty() {
            vec![]
        } else {
            self.actor_repository
                .find_many_by_ids_for_game(&form.actor_ids, &game.id().unwrap(), db_transaction)
                .await
                .map_err(|error| {
                    DomainError::from(AssignmentErrorKind::ActorNotFound).with_cause(error)
                })?
        };

        if form.assign_player {
            found_actors.push(ActorKind::Player(game.player().clone()));
        }

        self.assign_process_to_actors(
            found_actors,
            ProcessKind::CyclicProcess(found_process),
            db_transaction,
        )
        .await
    }
}

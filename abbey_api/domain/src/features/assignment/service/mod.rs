use sea_orm::ConnectionTrait;

use crate::{
    features::{
        actor::{domain::ActorKind, repository::ActorRepository, service::MonkCommandService},
        assignment::{
            domain::{ProcessAssignment, process_assignment_factory::ProcessAssignmentFactory},
            error::AssignmentErrorKind,
            forms::ProcessAssignmentForm,
        },
        game::domain::Game,
        player::service::PlayerCommandService,
        process::{
            domain::ProcessKind,
            service::{
                cyclic_process::{CyclicProcessCommandService, CyclicProcessQueryService},
                task::TaskCommandService,
            },
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a command service for Processes.
#[derive(Clone)]
pub struct ProcessCommandService {
    actor_repository: ActorRepository,
    player_command_service: PlayerCommandService,
    monk_command_service: MonkCommandService,
    cyclic_process_command_service: CyclicProcessCommandService,
    cyclic_process_query_service: CyclicProcessQueryService,
    task_command_service: TaskCommandService,
}

impl ProcessCommandService {
    /// Creates a new [`ProcessCommandService`].
    pub fn new(
        actor_repository: ActorRepository,
        player_command_service: PlayerCommandService,
        monk_command_service: MonkCommandService,
        cyclic_process_command_service: CyclicProcessCommandService,
        cyclic_process_query_service: CyclicProcessQueryService,
        task_command_service: TaskCommandService,
    ) -> Self {
        Self {
            actor_repository,
            player_command_service,
            monk_command_service,
            cyclic_process_command_service,
            cyclic_process_query_service,
            task_command_service,
        }
    }

    /// Assigns a [`Actor`][ActorKind] to a [`Process`][ProcessKind].
    async fn assign_process_to_actors<C: ConnectionTrait>(
        &self,
        actors: Vec<ActorKind>,
        process: ProcessKind,
        db_connection: &C,
    ) -> Result<ProcessAssignment, DomainError<AssignmentErrorKind>> {
        let assignment = ProcessAssignmentFactory::assign_process_to_actors(actors, process)?;

        for actor in assignment.actors() {
            match actor {
                ActorKind::Monk(monk) => {
                    self.monk_command_service
                        .update(monk.clone(), db_connection)
                        .await
                        .map_err(|error| {
                            DomainError::from(AssignmentErrorKind::ActorAssignment)
                                .with_cause(error)
                        })?;
                }
                ActorKind::Player(player) => {
                    self.player_command_service
                        .update(player.clone(), db_connection)
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
                self.cyclic_process_command_service
                    .update(cyclic_process.clone(), db_connection)
                    .await
                    .map_err(|error| {
                        DomainError::from(AssignmentErrorKind::ProcessAssignment).with_cause(error)
                    })?;
            }
            ProcessKind::Task(task) => {
                self.task_command_service
                    .update(task.clone(), db_connection)
                    .await
                    .map_err(|error| {
                        DomainError::from(AssignmentErrorKind::ProcessAssignment).with_cause(error)
                    })?;
            }
        }

        Ok(assignment)
    }

    /// Assigns a [`Process`][ProcessKind] based on the provided [ProcessAssignmentForm].
    pub async fn assign_process_to_actors_in_game<C: ConnectionTrait>(
        &self,
        form: ProcessAssignmentForm,
        game: &Game,
        db_connection: &C,
    ) -> Result<ProcessAssignment, DomainError<AssignmentErrorKind>> {
        let found_process = self
            .cyclic_process_query_service
            .get_by_id(&form.process_id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AssignmentErrorKind::ProcessNotFound).with_cause(error)
            })?;

        let mut found_actors = if form.actor_ids.is_empty() {
            vec![]
        } else {
            self.actor_repository
                .find_many_by_ids_for_game(&form.actor_ids, &game.id().unwrap(), db_connection)
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
            db_connection,
        )
        .await
    }
}

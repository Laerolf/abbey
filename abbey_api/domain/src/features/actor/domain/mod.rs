use crate::{
    features::{
        actor::{
            domain::{actor_status::ActorStatus, monk::Monk},
            error::ActorErrorKind,
        },
        player::domain::Player,
        process::domain::ProcessKind,
    },
    shared::error::DomainError,
};

pub mod actor_status;
pub mod monk;
pub mod person;

#[derive(Clone, Debug)]
pub enum ActorKind {
    Monk(Monk),
    Player(Player),
}

impl Actor for ActorKind {
    fn id(&self) -> &Option<i32> {
        match self {
            ActorKind::Monk(monk) => monk.id(),
            ActorKind::Player(player) => player.id(),
        }
    }

    fn status(&self) -> &ActorStatus {
        match self {
            ActorKind::Monk(monk) => monk.status(),
            ActorKind::Player(player) => player.status(),
        }
    }

    fn assigned_process(&self) -> &Option<ProcessKind> {
        match self {
            ActorKind::Monk(monk) => monk.assigned_process(),
            ActorKind::Player(player) => player.assigned_process(),
        }
    }

    fn assign_process(&mut self, process: ProcessKind) -> Result<(), DomainError<ActorErrorKind>> {
        match self {
            ActorKind::Monk(monk) => monk.assign_process(process),
            ActorKind::Player(player) => player.assign_process(process),
        }
    }

    fn unassign_process(&mut self) {
        match self {
            ActorKind::Monk(monk) => monk.unassign_process(),
            ActorKind::Player(player) => player.unassign_process(),
        }
    }
}

/// Represents an actor in the domain of the project.
pub trait Actor: Send {
    /// Gets the ID of the [`Actor`].
    fn id(&self) -> &Option<i32>;

    /// Gets the [status][`:ActorStatus`] of an [`Actor`].
    fn status(&self) -> &ActorStatus;

    /// Gets the [assigned process][`ProcessKind`] of an [`Actor`].
    fn assigned_process(&self) -> &Option<ProcessKind>;

    /// Assigns a [Process][`ProcessKind`] to an [`Actor`].
    fn assign_process(&mut self, process: ProcessKind) -> Result<(), DomainError<ActorErrorKind>>;

    /// Unassigns a [Process][`ProcessKind`] from an [`Actor`].
    fn unassign_process(&mut self);
}

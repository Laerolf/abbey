use crate::{
    features::{
        actor::{
            domain::{actor_status::ActorStatus, monk::Monk},
            error::ActorErrorKind,
        },
        player::domain::Player,
        process::domain::ProcessKind,
    },
    shared::{DomainElement, error::DomainError},
};

pub mod actor_status;
pub mod monk;
pub mod person;

#[derive(Clone, Debug)]
pub enum ActorKind {
    Monk(Monk),
    Player(Player),
}

impl ActorKind {
    pub fn id(&self) -> Result<i32, DomainError<ActorErrorKind>> {
        match self {
            ActorKind::Monk(monk) => monk.id(),
            ActorKind::Player(player) => player.id(),
        }
    }
}

impl Actor for ActorKind {
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
    /// Gets the [status][ActorStatus] of an [`Actor`].
    fn status(&self) -> &ActorStatus;

    /// Gets the [assigned process][ProcessKind] of an [`Actor`].
    fn assigned_process(&self) -> &Option<ProcessKind>;

    /// Assigns a [Process][ProcessKind] to an [`Actor`].
    fn assign_process(&mut self, process: ProcessKind) -> Result<(), DomainError<ActorErrorKind>>;

    /// Unassigns a [Process][ProcessKind] from an [`Actor`].
    fn unassign_process(&mut self);
}

/// Represents a link between a [`Process`][ProcessKind] and an [`Actor`][ActorKind].
pub struct ProcessActorLink {
    process_id: i32,
    actor: ActorKind,
}

impl ProcessActorLink {
    /// Creates a [`ProcessActorLink`] from the provided Process ID and Actor.
    pub fn from(process_id: i32, actor: ActorKind) -> Self {
        Self { process_id, actor }
    }

    pub fn process_id(&self) -> &i32 {
        &self.process_id
    }

    pub fn actor(&self) -> &ActorKind {
        &self.actor
    }
}

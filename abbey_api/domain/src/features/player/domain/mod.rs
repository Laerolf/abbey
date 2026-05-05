use time::OffsetDateTime;

use crate::{
    features::{
        actor::{
            domain::{Actor, actor_status::ActorStatus},
            error::ActorErrorKind,
        },
        process::domain::ProcessKind,
    },
    shared::{DomainElement, error::DomainError},
};

#[derive(Clone, Debug)]
pub struct Player {
    /// The ID of this [`Player`].
    id: Option<i32>,

    /// The creation date of this [`Player`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Player`].
    last_updated_at: Option<OffsetDateTime>,

    /// The assigned Process of this [`Player`].
    assigned_process: Option<ProcessKind>,
}

impl Player {
    /// Creates a new [`Player`].
    pub fn new(assigned_process: Option<ProcessKind>) -> Self {
        Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            assigned_process,
        }
    }

    /// Creates a [`Player`].
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        assigned_process: Option<ProcessKind>,
    ) -> Self {
        Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            assigned_process,
        }
    }

    /// Gets the  [assigned Process][ProcessKind] of this [`Player`].
    pub fn assigned_process(&self) -> &Option<ProcessKind> {
        &self.assigned_process
    }
}

impl Actor for Player {
    /// Gets the [status][`ActorStatus`] of a [`Player`].
    fn status(&self) -> &ActorStatus {
        if self.assigned_process.is_some() {
            return &ActorStatus::Assigned;
        }

        &ActorStatus::Available
    }

    /// Gets the [assigned process][`ProcessKind`] of a [`Player`].
    fn assigned_process(&self) -> &Option<ProcessKind> {
        &self.assigned_process
    }

    /// Assigns this [`Player`] to a [Process][`ProcessKind`].
    fn assign_process(&mut self, process: ProcessKind) -> Result<(), DomainError<ActorErrorKind>> {
        if *self.status() == ActorStatus::Assigned {
            return Err(DomainError::from(ActorErrorKind::Assigned));
        }

        self.assigned_process = Some(process);

        Ok(())
    }

    /// Unassigns this [`Player`] from a [Process][`crate::features::process::domain::Process`].
    fn unassign_process(&mut self) {
        self.assigned_process = None;
    }
}

impl DomainElement<ActorErrorKind> for Player {
    /// Gets the ID of this [`Player`].
    fn id(&self) -> Result<i32, DomainError<ActorErrorKind>> {
        self.id
            .ok_or(DomainError::from(ActorErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Player`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Player`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}

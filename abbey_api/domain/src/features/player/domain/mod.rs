use crate::{
    features::{
        actor::{
            domain::{Actor, actor_status::ActorStatus},
            error::ActorErrorKind,
        },
        process::domain::ProcessKind,
    },
    shared::error::DomainError,
};

#[derive(Clone, Debug)]
pub struct Player {
    /// The ID of this [`Player`].
    id: Option<i32>,

    /// The assigned Process of this [`Player`].
    assigned_process: Option<ProcessKind>,
}

impl Player {
    /// Creates a new [`Player`].
    pub fn new(assigned_process: Option<ProcessKind>) -> Self {
        Self {
            id: None,
            assigned_process,
        }
    }

    /// Creates a [`Player`].
    pub fn restore(id: i32, assigned_process: Option<ProcessKind>) -> Self {
        Self {
            id: Some(id),
            assigned_process,
        }
    }

    /// Gets the ID of this [`Player`].
    pub fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Gets the  [assigned Process][ProcessKind] of this [`Player`].
    pub fn assigned_process(&self) -> &Option<ProcessKind> {
        &self.assigned_process
    }
}

impl Actor for Player {
    /// Gets the ID of the [`Player`].
    fn id(&self) -> &Option<i32> {
        &self.id
    }

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

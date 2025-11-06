use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    features::{
        actor::{
            domain::{Actor, actor_status::ActorStatus},
            error::ActorError,
        },
        process::domain::Process,
    },
    shared::error::DomainError,
};

pub struct Player {
    /// The ID of this [`Player`].
    pub id: i32,

    /// The assigned Process of this [`Player`].
    pub assigned_process: Option<Weak<RefCell<dyn Process>>>,
}

impl Player {
    /// Creates a new [`Player`].
    pub fn new(id: i32, assigned_process: Option<Weak<RefCell<dyn Process>>>) -> Self {
        Self {
            id,
            assigned_process,
        }
    }
}

impl Actor for Player {
    /// Gets the [status][`crate::features::actor::domain::actor_status`] of a [`Player`].
    fn status(&self) -> ActorStatus {
        if self.assigned_process.is_some() {
            return ActorStatus::Assigned;
        }

        ActorStatus::Available
    }

    /// Assigns this [`Player`] to a [Process][`crate::features::process::domain::Process`].
    fn assign_process(
        &mut self,
        process: Rc<RefCell<dyn Process>>,
    ) -> Result<(), Box<dyn DomainError>> {
        if self.status() == ActorStatus::Assigned {
            return Err(Box::new(ActorError::Assigned));
        }

        self.assigned_process = Some(Rc::downgrade(&process));

        Ok(())
    }

    /// Unassigns this [`Player`] from a [Process][`crate::features::process::domain::Process`].
    fn unassign_process(&mut self) {
        self.assigned_process = None;
    }
}

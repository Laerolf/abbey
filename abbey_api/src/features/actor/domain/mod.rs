use std::{cell::RefCell, rc::Rc};

use crate::{
    features::{actor::domain::actor_status::ActorStatus, process::domain::Process},
    shared::error::DomainError,
};

pub mod actor_status;
pub mod monk;
pub mod person;

/// Represents an actor.
pub trait Actor {
    /// Gets the status of an actor.
    fn status(&self) -> ActorStatus;

    /// Assigns a [`crate::features::process::Process`] to an actor.
    fn assign_process(
        &mut self,
        process: Rc<RefCell<dyn Process>>,
    ) -> Result<(), Box<dyn DomainError>>;

    /// Unassigns a [`crate::features::process::Process`] from an actor.
    fn unassign_process(&mut self);
}

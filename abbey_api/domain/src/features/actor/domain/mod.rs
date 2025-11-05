use std::{cell::RefCell, rc::Rc};

use crate::{
    features::{actor::domain::actor_status::ActorStatus, process::domain::Process},
    shared::error::DomainError,
};

pub mod actor_status;
pub mod monk;
pub mod person;

/// Represents an actor in the domain of the project.
pub trait Actor {
    /// Gets the [status][`crate::features::actor::domain::ActorStatus`] of an [`Actor`].
    fn status(&self) -> ActorStatus;

    /// Assigns a [Process][`crate::features::process::domain::Process`] to an [`Actor`].
    fn assign_process(
        &mut self,
        process: Rc<RefCell<dyn Process>>,
    ) -> Result<(), Box<dyn DomainError>>;

    /// Unassigns a [Process][`crate::features::process::domain::Process`] from an [`Actor`].
    fn unassign_process(&mut self);
}

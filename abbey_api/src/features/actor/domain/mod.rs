use std::{cell::RefCell, rc::Rc};

use crate::features::process::domain::Process;

pub mod monk;
pub mod person;

/// Represents an actor.
pub trait Actor {
    /// Assigns a [`crate::features::process::Process`] to a person.
    fn assign_process(&mut self, process: Rc<RefCell<dyn Process>>);

    /// Unassigns a [`crate::features::process::Process`] from a person.
    fn unassign_process(&mut self);
}

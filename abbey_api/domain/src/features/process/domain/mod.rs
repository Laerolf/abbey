mod task;
use std::{cell::RefCell, rc::Rc};

pub use task::Task;

mod cyclic_process;
pub use cyclic_process::CyclicProcess;
use time::OffsetDateTime;

use crate::{
    features::{actor::domain::person::Person, output::domain::Output},
    shared::error::{DomainError, SharedError},
};

/// Represents a process.
pub trait Process {
    /// Asigns a [Person][`crate::features::actor::domain::person`] to this [`Process`].
    fn assign_person(&mut self, person: Rc<RefCell<dyn Person>>);

    /// Unassign a [Person][`crate::features::actor::domain::person`] from this [`Process`]:
    fn unassign_person(&mut self, person: &Rc<RefCell<dyn Person>>);

    /// Gets the [`Status`] of this [`Process`].
    fn status(&self) -> Status;

    /// Returns the ID of this [Process].
    fn id(&self) -> i32;

    /// Starts this [`Process`].
    fn start(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>>;

    /// Pauses this [`Process`].
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>>;

    /// Resumes this [`Process`].
    fn resume(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>>;

    /// Gets the [yield][`crate::features::output::domain::Output`] of this [`Process`].
    fn get_yield(&self) -> Option<Output>;

    /// Completes this [`Process`].
    fn complete(&mut self, _now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        Err(Box::new(SharedError::NotAvailable))
    }
}

/// Represents the [`Status`] of a [`Process`].
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Status {
    /// The [`Process`] has been created.
    New,
    /// The [`Process`] has started and is in progress.
    InProgress,
    /// The [`Process`] has been paused.
    Paused,
    /// The [`Process`] has been completed.
    Completed,
}

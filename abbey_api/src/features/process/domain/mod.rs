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
    /// Asigns a person to this process.
    fn assign_person(&mut self, person: Rc<RefCell<dyn Person>>);

    /// Unassign a person from this process:
    fn unassign_person(&mut self, person: &Rc<RefCell<dyn Person>>);

    /// Gets the status of this process.
    fn status(&self) -> Status;

    /// Starts this process.
    fn start(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>>;

    /// Pauses this process.
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>>;

    /// Resumes this process.
    fn resume(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>>;

    /// Gets the yield of this process.
    fn get_yield(&self) -> Option<Output>;

    /// Completes this process.
    fn complete(&mut self, _now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        Err(Box::new(SharedError::NotAvailable))
    }
}

/// Represents the status of a [`Process`].
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Status {
    /// The process has been created.
    New,
    /// The process has started and is in progress.
    InProgress,
    /// The process has been paused.
    Paused,
    /// The process has been completed.
    Completed,
}

mod task;
use std::{
    fmt::Display,
    str::FromStr,
    sync::{Arc, Mutex},
};

pub use task::Task;

mod cyclic_process;
pub use cyclic_process::CyclicProcess;
use time::OffsetDateTime;

use crate::{
    features::{actor::domain::person::Person, output::domain::Output},
    shared::error::{DomainErrorKind, SharedErrorKind},
};

/// Represents a process.
pub trait Process: Send {
    /// Asigns a [Person][`crate::features::actor::domain::person`] to this [`Process`].
    fn assign_person(&mut self, person: Arc<Mutex<dyn Person>>);

    /// Unassign a [Person][`crate::features::actor::domain::person`] from this [`Process`]:
    fn unassign_person(&mut self, person: &Arc<Mutex<dyn Person>>);

    /// Gets the [`Status`] of this [`Process`].
    fn status(&self) -> Status;

    /// Returns the ID of this [Process].
    fn id(&self) -> i32;

    /// Starts this [`Process`].
    fn start(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainErrorKind>>;

    /// Pauses this [`Process`].
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainErrorKind>>;

    /// Resumes this [`Process`].
    fn resume(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainErrorKind>>;

    /// Gets the [yield][`crate::features::output::domain::Output`] of this [`Process`].
    fn get_yield(&self) -> Option<Output>;

    /// Completes this [`Process`].
    fn complete(&mut self, _now: OffsetDateTime) -> Result<(), Box<dyn DomainErrorKind>> {
        Err(Box::new(SharedErrorKind::NotAvailable))
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

impl Status {
    /// Returns a string representing the [Status].
    fn as_str(&self) -> &'static str {
        match self {
            Self::New => "new",
            Self::InProgress => "in_progress",
            Self::Paused => "paused",
            Self::Completed => "completed",
        }
    }
}

impl FromStr for Status {
    type Err = String;

    /// Returns the [Status] represented by the provided value.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(Self::New),
            "in_progress" => Ok(Self::InProgress),
            "paused" => Ok(Self::Paused),
            "completed" => Ok(Self::Completed),
            _ => Err(format!("Invalid status: '{}'", s)),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

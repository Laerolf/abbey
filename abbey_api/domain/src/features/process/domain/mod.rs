use std::{any::Any, fmt::Display, str::FromStr};
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::domain::ActorKind,
        output::domain::Output,
        process::{
            domain::{cyclic_process::CyclicProcess, task::Task},
            error::ProcessErrorKind,
        },
    },
    shared::{DomainElement, error::DomainError},
};

pub mod cyclic_process;
pub mod task;

#[derive(Clone, Debug)]
pub enum ProcessKind {
    CyclicProcess(CyclicProcess),
    Task(Task),
}

impl ProcessKind {
    pub fn id(&self) -> Result<i32, DomainError<ProcessErrorKind>> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.id(),
            ProcessKind::Task(task) => task.id(),
        }
    }
}

impl Process for ProcessKind {
    fn as_any(&self) -> &dyn Any {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.as_any(),
            ProcessKind::Task(task) => task.as_any(),
        }
    }

    fn assign_person(&mut self, person: ActorKind) {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.assign_person(person),
            ProcessKind::Task(task) => task.assign_person(person),
        }
    }

    fn unassign_person(&mut self, person: &ActorKind) {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.unassign_person(person),
            ProcessKind::Task(task) => task.unassign_person(person),
        }
    }

    fn status(&self) -> &Status {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.status(),
            ProcessKind::Task(task) => task.status(),
        }
    }

    fn started_at(&self) -> &Option<OffsetDateTime> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.started_at(),
            ProcessKind::Task(task) => task.started_at(),
        }
    }

    fn paused_at(&self) -> &Option<OffsetDateTime> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.paused_at(),
            ProcessKind::Task(task) => task.paused_at(),
        }
    }

    fn elapsed(&self) -> &Duration {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.elapsed(),
            ProcessKind::Task(task) => task.elapsed(),
        }
    }

    fn assigned_actors(&self) -> &Vec<ActorKind> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.assigned_actors(),
            ProcessKind::Task(task) => task.assigned_actors(),
        }
    }

    fn start(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.start(now),
            ProcessKind::Task(task) => task.start(now),
        }
    }

    fn pause(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.pause(now),
            ProcessKind::Task(task) => task.pause(now),
        }
    }

    fn resume(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.resume(now),
            ProcessKind::Task(task) => task.resume(now),
        }
    }

    fn get_yield(&self) -> Option<Output> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.get_yield(),
            ProcessKind::Task(task) => task.get_yield(),
        }
    }

    fn complete(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        match self {
            ProcessKind::CyclicProcess(cyclic_process) => cyclic_process.complete(now),
            ProcessKind::Task(task) => task.complete(now),
        }
    }
}

/// Represents a process.
pub trait Process: Send + Any {
    /// Used to downcast a [`Process`].
    fn as_any(&self) -> &dyn Any;

    /// Asigns a [Person][`ActorKind`] to this [`Process`].
    fn assign_person(&mut self, person: ActorKind);

    /// Unassign a [Person][`ActorKind`] from this [`Process`]:
    fn unassign_person(&mut self, person: &ActorKind);

    /// Gets the [`Status`] of this [`Process`].
    fn status(&self) -> &Status;

    /// Returns the time when this [Process] was last started.
    fn started_at(&self) -> &Option<OffsetDateTime>;

    /// Returns the time when this [Process] was last paused.
    fn paused_at(&self) -> &Option<OffsetDateTime>;

    /// Returns the time this [Process] ran.
    fn elapsed(&self) -> &Duration;

    /// Returns the [Actors][Vec<ActorKind>] assigned to this [Process].
    fn assigned_actors(&self) -> &Vec<ActorKind>;

    /// Starts this [`Process`].
    fn start(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>>;

    /// Pauses this [`Process`].
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>>;

    /// Resumes this [`Process`].
    fn resume(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>>;

    /// Gets the [yield][`Output`] of this [`Process`].
    fn get_yield(&self) -> Option<Output>;

    /// Completes this [`Process`].
    fn complete(&mut self, _now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        Err(DomainError::from(ProcessErrorKind::NotFound))
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

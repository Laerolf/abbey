use std::{cell::RefCell, rc::Rc};

use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{
    features::{
        actor::domain::person::Person, output::domain::Output, process::error::ProcessError,
    },
    shared::error::DomainError,
};

use super::{Process, Status};

/// Represents a [`super::Process`] that starts at a certain time and ends after a [`time::Duration`] has passed.
pub struct Task {
    /// The ID of this task.
    pub id: Uuid,

    /// The status of this task.
    pub status: Status,

    /// The time this task started.
    pub started_at: Option<OffsetDateTime>,

    /// The time this task was paused.
    pub paused_at: Option<OffsetDateTime>,

    /// The duration of this task.
    pub duration: Duration,

    /// The time that has elapsed since the task was started.
    pub elapsed: Duration,

    /// The people assigned to this task.
    pub assigned_people: Vec<Rc<RefCell<dyn Person>>>,
}

impl Task {
    /// Creates a new `Task` based on the provided [`time::Duration`].
    pub fn new(duration: Duration) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: Status::New,
            started_at: None,
            paused_at: None,
            duration,
            elapsed: Duration::ZERO,
            assigned_people: Vec::new(),
        }
    }

    /// Gets the end time of this task based on its start time, duration, the elapsed time since this task was started and the current stime.
    pub fn ends_at(&self, now: OffsetDateTime) -> Option<OffsetDateTime> {
        match (self.status, self.started_at) {
            (Status::InProgress, Some(started_at)) => started_at
                .checked_add(self.duration)
                .and_then(|ends_at| ends_at.checked_sub(self.elapsed)),
            (Status::Paused, _) => Some(now + (self.duration - self.elapsed)),
            _ => None,
        }
    }

    /// Gets the progress of this task.
    pub fn progress(&self, now: OffsetDateTime) -> f32 {
        let task_elapsed_duration: Duration = match (self.started_at, self.status) {
            (Some(started_at), Status::InProgress) => self.elapsed + (now - started_at),
            (_, Status::Paused) => self.elapsed,
            _ => Duration::ZERO,
        };

        (task_elapsed_duration.as_seconds_f32() / self.duration.as_seconds_f32()).clamp(0.00, 1.00)
    }
}

impl Process for Task {
    /// Assigns a person to this task.
    fn assign_person(&mut self, person: Rc<RefCell<dyn Person>>) {
        self.assigned_people.push(person);
    }

    /// Unassigns a person to this task.
    fn unassign_person(&mut self, person: &Rc<RefCell<dyn Person>>) {
        self.assigned_people.retain(|p| !Rc::ptr_eq(p, person));
    }

    /// Gets the status of this task.
    fn status(&self) -> Status {
        self.status
    }

    /// Starts this task.
    fn start(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::New {
            return Err(Box::new(ProcessError::NotNew));
        } else if self.assigned_people.is_empty() {
            return Err(Box::new(ProcessError::NoAssignedPeople));
        }

        self.status = Status::InProgress;
        self.started_at = Some(now);
        Ok(())
    }

    /// Pauses this task.
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::InProgress {
            return Err(Box::new(ProcessError::NotInProgress));
        }

        if let Some(started_at) = self.started_at {
            let task_duration = now - started_at;
            self.elapsed += task_duration;
            self.paused_at = Some(now);
            self.status = Status::Paused;
            self.started_at = None;
        }

        Ok(())
    }

    /// Resumes this task.
    fn resume(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::Paused {
            return Err(Box::new(ProcessError::NotPaused));
        } else if self.assigned_people.is_empty() {
            return Err(Box::new(ProcessError::NoAssignedPeople));
        }

        self.started_at = Some(now);
        self.status = Status::InProgress;
        self.paused_at = None;
        Ok(())
    }

    /// Completes this task if possible.
    fn complete(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::New {
            return Err(Box::new(ProcessError::NotNew));
        } else if self.assigned_people.is_empty() {
            return Err(Box::new(ProcessError::NoAssignedPeople));
        }

        if self.progress(now) >= 1.0 {
            self.status = Status::Completed;
            self.started_at = None;
            self.paused_at = None;
            Ok(())
        } else {
            Err(Box::new(ProcessError::NotComplete))
        }
    }

    /// Gets the output of this task.
    fn get_yield(&self) -> Option<Output> {
        None
    }
}

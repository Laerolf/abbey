use std::{cell::RefCell, rc::Rc};

use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::domain::person::Person, output::domain::Output, process::error::ProcessError,
    },
    shared::error::DomainError,
};

use super::{Process, Status};

/// Represents a [Process][`super::Process`] that starts at a certain time and ends after a [`time::Duration`] has passed.
pub struct Task {
    /// The ID of this [`Task`].
    pub id: i32,

    /// The [Status][`crate::features::process::domain::Status`] of this [`Task`].
    pub status: Status,

    /// The time this [`Task`] started.
    pub started_at: Option<OffsetDateTime>,

    /// The time this [`Task`] was paused.
    pub paused_at: Option<OffsetDateTime>,

    /// The duration of this [`Task`].
    pub duration: Duration,

    /// The time that has elapsed since the [`Task`] was started.
    pub elapsed: Duration,

    /// The [People][`crate::features::actor::domain::person`] assigned to this [`Task`].
    pub assigned_people: Vec<Rc<RefCell<dyn Person>>>,
}

impl Task {
    /// Creates a new [`Task`] based on the provided [`time::Duration`].
    pub fn new(id: i32, duration: Duration) -> Self {
        Self {
            id,
            status: Status::New,
            started_at: None,
            paused_at: None,
            duration,
            elapsed: Duration::ZERO,
            assigned_people: Vec::new(),
        }
    }

    /// Gets the end time of this [`Task`] based on its start time, duration, the elapsed time since this [`Task`] was started and the current stime.
    pub fn ends_at(&self, now: OffsetDateTime) -> Option<OffsetDateTime> {
        match (self.status, self.started_at) {
            (Status::InProgress, Some(started_at)) => started_at
                .checked_add(self.duration)
                .and_then(|ends_at| ends_at.checked_sub(self.elapsed)),
            (Status::Paused, _) => Some(now + (self.duration - self.elapsed)),
            _ => None,
        }
    }

    /// Gets the progress of this [`Task`].
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
    /// Assigns a [Person][`crate::features::actor::domain::person`] to this [`Task`].
    fn assign_person(&mut self, person: Rc<RefCell<dyn Person>>) {
        self.assigned_people.push(person);
    }

    /// Unassigns a [Person][`crate::features::actor::domain::person`] to this [`Task`].
    fn unassign_person(&mut self, person: &Rc<RefCell<dyn Person>>) {
        self.assigned_people.retain(|p| !Rc::ptr_eq(p, person));
    }

    /// Gets the ID of this [`Task`].
    fn id(&self) -> i32 {
        self.id
    }

    /// Gets the [Status][`crate::features::process::domain::Status`] of this [`Task`].
    fn status(&self) -> Status {
        self.status
    }

    /// Starts this [`Task`].
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

    /// Pauses this [`Task`].
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

    /// Resumes this [`Task`].
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

    /// Completes this [`Task`] if possible.
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

    /// Gets the [Output][`crate::features::output::domain::Output`] of this [`Task`].
    fn get_yield(&self) -> Option<Output> {
        None
    }
}

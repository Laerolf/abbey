use std::any::Any;

use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::domain::{Actor, ActorKind},
        output::domain::{Output, resource::Resource},
        process::error::ProcessErrorKind,
    },
    shared::error::DomainError,
};

use super::{Process, Status};

/// The state of a [`Task`].
pub struct TaskState {
    pub status: Status,
    pub input_resources: Vec<Resource>,
    pub output_resources: Vec<Resource>,
    pub started_at: Option<OffsetDateTime>,
    pub paused_at: Option<OffsetDateTime>,
    pub duration: Duration,
    pub elapsed: Duration,
    pub assigned_people: Vec<ActorKind>,
}

/// Represents a [Process][`super::Process`] that starts at a certain time and ends after a [`time::Duration`] has passed.
#[derive(Clone, Debug)]
pub struct Task {
    /// The ID of this [`Task`].
    id: Option<i32>,

    /// The [Status] of this [`Task`].
    status: Status,

    /// The [Resources][Vec<Resource>] required for this [`Task`] to start.
    input_resources: Vec<Resource>,

    /// The [Resources][Vec<Resource>] outputted by this [`Task`].
    output_resources: Vec<Resource>,

    /// The time this [`Task`] started.
    started_at: Option<OffsetDateTime>,

    /// The time this [`Task`] was paused.
    paused_at: Option<OffsetDateTime>,

    /// The duration of this [`Task`].
    duration: Duration,

    /// The time that has elapsed since the [`Task`] was started.
    elapsed: Duration,

    /// The [People][`ActorKind`] assigned to this [`Task`].
    assigned_people: Vec<ActorKind>,
}

impl Task {
    /// Creates a new [`Task`] based on the provided [`time::Duration`] and [Resources][Vec<Resource>].
    pub fn new(
        input_resources: Vec<Resource>,
        output_resources: Vec<Resource>,
        duration: Duration,
    ) -> Result<Self, DomainError<ProcessErrorKind>> {
        if input_resources.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoInputResources));
        } else if output_resources.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoOutputResources));
        };

        Ok(Self {
            id: None,
            status: Status::New,
            input_resources,
            output_resources,
            started_at: None,
            paused_at: None,
            duration,
            elapsed: Duration::ZERO,
            assigned_people: Vec::new(),
        })
    }

    /// Creates a [`Task`] based on the provided parameters.
    pub fn restore(id: i32, state: TaskState) -> Result<Self, DomainError<ProcessErrorKind>> {
        if state.input_resources.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoInputResources));
        } else if state.output_resources.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoOutputResources));
        };

        Ok(Self {
            id: Some(id),
            status: state.status,
            input_resources: state.input_resources,
            output_resources: state.output_resources,
            started_at: state.started_at,
            paused_at: state.paused_at,
            duration: state.duration,
            elapsed: state.elapsed,
            assigned_people: state.assigned_people,
        })
    }

    pub fn input_resources(&self) -> &Vec<Resource> {
        &self.input_resources
    }

    pub fn output_resources(&self) -> &Vec<Resource> {
        &self.output_resources
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    pub fn elapsed(&self) -> &Duration {
        &self.elapsed
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
    /// Used to downcast to a [`Task`].
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Assigns a [Person][`ActorKind`] to this [`Task`].
    fn assign_person(&mut self, person: ActorKind) {
        self.assigned_people.push(person);
    }

    /// Unassigns a [Person][`ActorKind`] to this [`Task`].
    fn unassign_person(&mut self, actor_to_unassign: &ActorKind) {
        self.assigned_people
            .retain(|actor| actor.id() != actor_to_unassign.id());
    }

    /// Gets the ID of this [`Task`].
    fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Gets the [Status] of this [`Task`].
    fn status(&self) -> &Status {
        &self.status
    }

    /// Gets the time this [`Task`] was last started.
    fn started_at(&self) -> &Option<OffsetDateTime> {
        &self.started_at
    }

    /// Gets the time this [`Task`] was last paused.
    fn paused_at(&self) -> &Option<OffsetDateTime> {
        &self.paused_at
    }

    /// Returns the time this [`Task`] ran.
    fn elapsed(&self) -> &Duration {
        &self.elapsed
    }

    /// Starts this [`Task`].
    fn start(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        if self.status != Status::New {
            return Err(DomainError::from(ProcessErrorKind::NotNew));
        } else if self.assigned_people.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoAssignedPeople));
        }

        self.status = Status::InProgress;
        self.started_at = Some(now);

        Ok(())
    }

    /// Pauses this [`Task`].
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        if self.status != Status::InProgress {
            return Err(DomainError::from(ProcessErrorKind::NotInProgress));
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
    fn resume(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        if self.status != Status::Paused {
            return Err(DomainError::from(ProcessErrorKind::NotPaused));
        } else if self.assigned_people.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoAssignedPeople));
        }

        self.started_at = Some(now);
        self.status = Status::InProgress;
        self.paused_at = None;

        Ok(())
    }

    /// Completes this [`Task`] if possible.
    fn complete(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        if self.status != Status::New {
            return Err(DomainError::from(ProcessErrorKind::NotNew));
        } else if self.assigned_people.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoAssignedPeople));
        }

        if self.progress(now) >= 1.0 {
            self.status = Status::Completed;
            self.started_at = None;
            self.paused_at = None;

            Ok(())
        } else {
            Err(DomainError::from(ProcessErrorKind::NotComplete))
        }
    }

    /// Gets the [Output] of this [`Task`].
    fn get_yield(&self) -> Option<Output> {
        None
    }
}

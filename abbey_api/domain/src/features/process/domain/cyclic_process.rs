use std::any::Any;

use rand::seq::SliceRandom;
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::domain::ActorKind,
        output::domain::{Output, resource::Resource},
        process::error::ProcessErrorKind,
    },
    shared::{DomainElement, error::DomainError},
};

use super::{Process, Status};

/// The state of a [`CyclicProcess`].
pub struct CyclicProcessState {
    pub status: Status,
    pub output_resources: Vec<Resource>,
    pub started_at: Option<OffsetDateTime>,
    pub paused_at: Option<OffsetDateTime>,
    pub cycle_interval: Duration,
    pub elapsed: Duration,
    pub assigned_people: Vec<ActorKind>,
}

/// Represents a [`super::Process`] with cycles that have an interval.
#[derive(Clone, Debug)]
pub struct CyclicProcess {
    /// The ID of this [`CyclicProcess`].
    id: Option<i32>,

    /// The creation date of this [`CyclicProcess`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`CyclicProcess`].
    last_updated_at: Option<OffsetDateTime>,

    /// The [Status] of this [`CyclicProcess`].
    status: Status,

    /// The possible [Resources][Vec<Resource>] outputted by this [`CyclicProcess`].
    output_resources: Vec<Resource>,

    /// The time this [`CyclicProcess`] was started last.
    started_at: Option<OffsetDateTime>,

    /// The time this [`CyclicProcess`] was paused last.
    paused_at: Option<OffsetDateTime>,

    /// The cycle interval of this [`CyclicProcess`].
    cycle_interval: Duration,

    /// The time that has elapsed since this [`CyclicProcess`] was started.
    elapsed: Duration,

    /// The [People][`ActorKind`] assigned to this [`CyclicProcess`].
    assigned_people: Vec<ActorKind>,
}

impl CyclicProcess {
    /// Creates a new [`CyclicProcess`] based on the provided parameters.
    pub fn new(
        output_resources: Vec<Resource>,
        cycle_interval: Duration,
    ) -> Result<Self, DomainError<ProcessErrorKind>> {
        if output_resources.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoOutputResources));
        };

        Ok(Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            status: Status::New,
            output_resources,
            started_at: None,
            paused_at: None,
            cycle_interval,
            elapsed: Duration::seconds(0),
            assigned_people: Vec::new(),
        })
    }

    /// Creates a [`CyclicProcess`] based on the provided parameters.
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        state: CyclicProcessState,
    ) -> Result<Self, DomainError<ProcessErrorKind>> {
        if state.output_resources.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoOutputResources));
        };

        Ok(Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            status: state.status,
            output_resources: state.output_resources,
            started_at: state.started_at,
            paused_at: state.paused_at,
            cycle_interval: state.cycle_interval,
            elapsed: state.elapsed,
            assigned_people: state.assigned_people,
        })
    }

    pub fn output_resources(&self) -> &Vec<Resource> {
        &self.output_resources
    }

    /// Returns the duration of a cycle of this [`CyclicProcess`].
    pub fn cycle_interval(&self) -> &Duration {
        &self.cycle_interval
    }

    /// Calculates the completed cycles since the provided time.
    pub fn completed_cycles(&self, last_cycle_check: OffsetDateTime) -> u32 {
        let cycle_elapsed_duration: Duration = match (self.started_at, self.status) {
            (Some(started_at), Status::InProgress) => {
                self.elapsed + (last_cycle_check - started_at)
            }
            (_, Status::Paused) => self.elapsed,
            _ => Duration::ZERO,
        };

        (cycle_elapsed_duration.as_seconds_f32() / self.cycle_interval.as_seconds_f32()).floor()
            as u32
    }
}

impl Process for CyclicProcess {
    /// Used to downcast to a [`CyclicProcess`].
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Assigns a [Person][`ActorKind`] to this [`CyclicProcess`].
    fn assign_person(&mut self, person: ActorKind) {
        self.assigned_people.push(person);
    }

    /// Unassigns a [Person][`ActorKind`] to this [`CyclicProcess`].
    fn unassign_person(&mut self, actor_to_unassign: &ActorKind) {
        self.assigned_people
            .retain(|actor| actor.id().unwrap() != actor_to_unassign.id().unwrap());
    }

    /// Gets the [Status] of this [`CyclicProcess`].
    fn status(&self) -> &Status {
        &self.status
    }

    /// Gets the time this [`CyclicProcess`] was last started.
    fn started_at(&self) -> &Option<OffsetDateTime> {
        &self.started_at
    }

    /// Gets the time this [`CyclicProcess`] was last paused.
    fn paused_at(&self) -> &Option<OffsetDateTime> {
        &self.paused_at
    }

    /// Returns the time this [`CyclicProcess`] ran.
    fn elapsed(&self) -> &Duration {
        &self.elapsed
    }

    /// Returns the [Actors][Vec<ActorKind>] that was assigned to this [`CyclicProcess`].
    fn assigned_actors(&self) -> &Vec<ActorKind> {
        &self.assigned_people
    }

    /// Starts this [`CyclicProcess`].
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

    /// Pauses this [`CyclicProcess`].
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), DomainError<ProcessErrorKind>> {
        if self.status != Status::InProgress {
            return Err(DomainError::from(ProcessErrorKind::NotInProgress));
        }

        if let Some(started_at) = self.started_at {
            let cycle_duration = now - started_at;
            self.elapsed += cycle_duration;
            self.paused_at = Some(now);
            self.status = Status::Paused;
            self.started_at = None;
        }

        Ok(())
    }

    /// Resumes this [`CyclicProcess`].
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

    /// Gets the [Output][`crate::features::output::domain::Output`] of a cycle of this [`CyclicProcess`].
    fn get_yield(&self) -> Option<Output> {
        let mut random_number_generator = rand::thread_rng();
        // TODO: Use weights
        let resource_range: Vec<usize> = (0..(self.output_resources.len())).collect();

        let quantity: i32 = self.assigned_people.len().try_into().unwrap_or(0);

        if let Some(selected_resource_index) = resource_range.choose(&mut random_number_generator) {
            self.output_resources
                .get(*selected_resource_index)
                // TODO: Use dynamic amounts
                .map(|selected_resource| Output::new(selected_resource.clone(), quantity))
        } else {
            None
        }
    }
}

impl DomainElement<ProcessErrorKind> for CyclicProcess {
    /// Gets the ID of this [`CyclicProcess`].
    fn id(&self) -> Result<i32, DomainError<ProcessErrorKind>> {
        self.id
            .ok_or(DomainError::from(ProcessErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`CyclicProcess`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`CyclicProcess`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}

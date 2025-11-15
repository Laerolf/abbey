use std::{cell::RefCell, rc::Rc};

use rand::seq::SliceRandom;
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::domain::person::Person,
        output::domain::{Output, resource::Resource},
        process::error::ProcessError,
    },
    shared::error::DomainError,
};

use super::{Process, Status};

/// Represents a [`super::Process`] with cycles that have an interval.
pub struct CyclicProcess {
    /// The ID of this [`CyclicProcess`].
    pub id: i32,

    /// The [Status][`super::Status`] of this [`CyclicProcess`].
    pub status: Status,

    /// The possible [Resources][`crate::features::output::domain::resource`] outputted by this [`CyclicProcess`].
    pub output_resources: Vec<Resource>,

    /// The time this [`CyclicProcess`] was started last.
    pub started_at: Option<OffsetDateTime>,

    /// The time this [`CyclicProcess`] was paused last.
    pub paused_at: Option<OffsetDateTime>,

    /// The cycle interval of this [`CyclicProcess`].
    pub cycle_interval: Duration,

    /// The time that has elapsed since this [`CyclicProcess`] was started.
    pub elapsed: Duration,

    /// The [People][`crate::features::actor::domain::person`] assigned to this [`CyclicProcess`].
    pub assigned_people: Vec<Rc<RefCell<dyn Person>>>,
}

impl CyclicProcess {
    /// Creates a new [`CyclicProcess`] based on the provided cycle [`time::Duration`].
    pub fn new(
        id: i32,
        status: Status,
        output_resources: Vec<Resource>,
        started_at: Option<OffsetDateTime>,
        paused_at: Option<OffsetDateTime>,
        cycle_interval: Duration,
        elapsed: Duration,
        assigned_people: Vec<Rc<RefCell<dyn Person>>>,
    ) -> Self {
        Self {
            id,
            status,
            output_resources,
            started_at,
            paused_at,
            cycle_interval,
            elapsed,
            assigned_people,
        }
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
    /// Assigns a [Person][`crate::features::actor::domain::person`] to this [`CyclicProcess`].
    fn assign_person(&mut self, person: Rc<RefCell<dyn Person>>) {
        self.assigned_people.push(person);
    }

    /// Unassigns a [Person][`crate::features::actor::domain::person`] to this [`CyclicProcess`].
    fn unassign_person(&mut self, person: &Rc<RefCell<dyn Person>>) {
        self.assigned_people.retain(|p| !Rc::ptr_eq(p, person));
    }

    /// Gets the ID of this [`CyclicProcess`].
    fn id(&self) -> i32 {
        self.id
    }

    /// Gets the [Status][`super::Status`] of this [`CyclicProcess`].
    fn status(&self) -> Status {
        self.status
    }

    // Starts this [`CyclicProcess`].
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

    // Pauses this [`CyclicProcess`].
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::InProgress {
            return Err(Box::new(ProcessError::NotInProgress));
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

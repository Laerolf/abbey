use std::{cell::RefCell, rc::Rc};

use rand::seq::SliceRandom;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{
    features::{
        actor::domain::person::Person,
        output::domain::{resource::Resource, Output},
        process::error::ProcessError,
    },
    shared::error::DomainError,
};

use super::{Process, Status};

/// Represents a [`super::Process`] with cycles that have an interval.
pub struct CyclicProcess {
    /// The ID of this cyclic process.
    pub id: Uuid,

    /// The status of this cyclic process.
    pub status: Status,

    /// The possible resources outputted by this cyclic process.
    output_resources: Vec<Resource>,

    /// The time this cyclic process was started last.
    pub started_at: Option<OffsetDateTime>,

    /// The time this cyclic process was paused last.
    pub paused_at: Option<OffsetDateTime>,

    /// The cycle interval of this cyclic process.
    pub cycle_interval: Duration,

    /// The time that has elapsed since this cyclic process was started.
    pub elapsed: Duration,

    /// The people assigned to this cycle process.
    pub assigned_people: Vec<Rc<RefCell<dyn Person>>>,
}

impl CyclicProcess {
    /// Creates a new `CycleProcess` based on the provided cycle [`time::Duration`].
    pub fn new(cycle_interval: Duration, output_resources: Vec<Resource>) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: Status::New,
            output_resources,
            started_at: None,
            paused_at: None,
            cycle_interval,
            elapsed: Duration::ZERO,
            assigned_people: Vec::new(),
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
    /// Assigns a person to this cycle process.
    fn assign_person(&mut self, person: Rc<RefCell<dyn Person>>) {
        self.assigned_people.push(person);
    }

    /// Unassigns a person to this cycle process.
    fn unassign_person(&mut self, person: &Rc<RefCell<dyn Person>>) {
        self.assigned_people.retain(|p| !Rc::ptr_eq(p, person));
    }

    /// Gets the status of this cyclic process.
    fn status(&self) -> Status {
        self.status
    }

    // Starts this cyclic process.
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

    // Pauses this cyclic process.
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

    /// Resumes this cyclic process.
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

    /// Gets the output of a cycle of this cyclic process.
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

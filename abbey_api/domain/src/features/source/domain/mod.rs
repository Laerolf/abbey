use std::{cell::RefCell, rc::Rc};

use time::OffsetDateTime;

use crate::{
    features::{
        output::domain::Output,
        process::domain::{CyclicProcess, Process},
    },
    shared::error::DomainError,
};

/// Represents a source.
pub struct Source {
    /// The ID of this [`Source`].
    pub id: i32,

    /// The name of this [`Source`].
    pub name: String,

    /// The [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub process: Rc<RefCell<CyclicProcess>>,

    /// The last time a claim was made for the output of the completed cycles of this [`Source`].
    pub last_claim_at: Option<OffsetDateTime>,
}

impl Source {
    /// Creates a new [`Source`] based on the provided parameters.
    pub fn new(id: i32, name: impl Into<String>, process: CyclicProcess) -> Self {
        Self {
            id,
            name: name.into(),
            process: Rc::new(RefCell::new(process)),
            last_claim_at: None,
        }
    }

    /// Starts the [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn start_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        return self.process.borrow_mut().start(now);
    }

    /// Pauses the [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn pause_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        return self.process.borrow_mut().pause(now);
    }

    /// Resumes the [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn resume_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        return self.process.borrow_mut().resume(now);
    }

    /// Claims the output of this [`Source`]'s completed [Process][`crate::features::process::domain::CyclicProcess`] cycles.
    pub fn claim(&mut self, now: OffsetDateTime) -> Vec<Option<Output>> {
        let completed_cycles_since_last_claim = self
            .process
            .borrow_mut()
            .completed_cycles(self.last_claim_at.unwrap_or(now));

        self.last_claim_at = Some(now);

        if completed_cycles_since_last_claim == 0 {
            return Vec::new();
        }

        (0..completed_cycles_since_last_claim)
            .map(|_| self.process.borrow_mut().get_yield())
            .collect()
    }
}

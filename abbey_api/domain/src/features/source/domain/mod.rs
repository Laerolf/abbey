use std::sync::{Arc, Mutex};

use time::OffsetDateTime;

use crate::{
    features::{
        output::domain::Output,
        process::domain::{CyclicProcess, Process},
    },
    shared::error::DomainErrorKind,
};

/// Represents a source.
pub struct Source {
    /// The ID of this [`Source`].
    pub id: i32,

    /// The name of this [`Source`].
    pub name: String,

    /// The [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub process: Arc<Mutex<CyclicProcess>>,

    /// The last time a claim was made for the output of the completed cycles of this [`Source`].
    pub last_claim_at: Option<OffsetDateTime>,
}

impl Source {
    /// Creates a new [`Source`] based on the provided parameters.
    pub fn new(id: i32, name: impl Into<String>, process: CyclicProcess) -> Self {
        Self {
            id,
            name: name.into(),
            process: Arc::new(Mutex::new(process)),
            last_claim_at: None,
        }
    }

    /// Starts the [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn start_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainErrorKind>> {
        return self.process.lock().unwrap().start(now);
    }

    /// Pauses the [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn pause_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainErrorKind>> {
        return self.process.lock().unwrap().pause(now);
    }

    /// Resumes the [Process][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn resume_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainErrorKind>> {
        return self.process.lock().unwrap().resume(now);
    }

    /// Claims the output of this [`Source`]'s completed [Process][`crate::features::process::domain::CyclicProcess`] cycles.
    pub fn claim(&mut self, now: OffsetDateTime) -> Vec<Option<Output>> {
        let completed_cycles_since_last_claim = self
            .process
            .lock()
            .unwrap()
            .completed_cycles(self.last_claim_at.unwrap_or(now));

        self.last_claim_at = Some(now);

        if completed_cycles_since_last_claim == 0 {
            return Vec::new();
        }

        (0..completed_cycles_since_last_claim)
            .map(|_| self.process.lock().unwrap().get_yield())
            .collect()
    }
}

use time::OffsetDateTime;

use crate::{
    features::{
        output::domain::Output,
        process::{
            domain::{Process, cyclic_process::CyclicProcess},
            error::ProcessErrorKind,
        },
        source::error::SourceErrorKind,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a source.
#[derive(Clone)]
pub struct Source {
    /// The ID of this [`Source`].
    id: Option<i32>,

    /// The creation date of this [`Source`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Source`].
    last_updated_at: Option<OffsetDateTime>,

    /// The name of this [`Source`].
    name: String,

    /// The [CyclicProcess][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    process: CyclicProcess,

    /// The last time a claim was made for the output of the completed cycles of this [`Source`].
    last_claim_at: Option<OffsetDateTime>,
}

impl Source {
    /// Creates a new [`Source`].
    pub fn new(name: impl Into<String>, process: CyclicProcess) -> Self {
        Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            name: name.into(),
            process,
            last_claim_at: None,
        }
    }

    /// Creates a [`Source`] based on the provided parameters.
    pub fn from(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        name: impl Into<String>,
        process: CyclicProcess,
    ) -> Self {
        Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            name: name.into(),
            process,
            last_claim_at: None,
        }
    }

    /// Gets the name of this [`Source`].
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Gets the [Process][`CyclicProcess`] of this [`Source`].
    pub fn process(&self) -> &CyclicProcess {
        &self.process
    }

    /// Gets the [date of the last claim][`Option<OffsetDateTime>`] of this [`Source`].
    pub fn last_claim_at(&self) -> &Option<OffsetDateTime> {
        &self.last_claim_at
    }

    /// Starts the [CyclicProcess][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn start_fetching(
        &mut self,
        now: OffsetDateTime,
    ) -> Result<(), DomainError<SourceErrorKind>> {
        self.process
            .start(now)
            .map_err(|error| DomainError::from(SourceErrorKind::StartFetching).with_cause(error))?;

        Ok(())
    }

    /// Pauses the [CyclicProcess][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn pause_fetching(
        &mut self,
        now: OffsetDateTime,
    ) -> Result<(), DomainError<ProcessErrorKind>> {
        self.process.pause(now)?;

        Ok(())
    }

    /// Resumes the [CyclicProcess][`crate::features::process::domain::CyclicProcess`] of this [`Source`].
    pub fn resume_fetching(
        &mut self,
        now: OffsetDateTime,
    ) -> Result<(), DomainError<ProcessErrorKind>> {
        self.process.resume(now)?;

        Ok(())
    }

    /// Claims the output of this [`Source`]'s completed [CyclicProcess][`crate::features::process::domain::CyclicProcess`] cycles.
    pub fn claim(&mut self, now: OffsetDateTime) -> Vec<Option<Output>> {
        let completed_cycles = self
            .process
            .completed_cycles(self.last_claim_at.unwrap_or(now));
        self.last_claim_at = Some(now);

        if completed_cycles == 0 {
            return Vec::new();
        }

        (0..completed_cycles)
            .map(|_| self.process.get_yield())
            .collect()
    }
}

impl DomainElement<SourceErrorKind> for Source {
    /// Gets the ID of this [`Source`].
    fn id(&self) -> Result<i32, DomainError<SourceErrorKind>> {
        self.id
            .ok_or(DomainError::from(SourceErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Source`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Source`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}

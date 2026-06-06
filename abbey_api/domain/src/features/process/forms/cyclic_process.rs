use time::Duration;

use crate::{features::process::error::ProcessErrorKind, shared::error::DomainError};

/// Represents a CyclicProcess blueprint.
#[derive(Clone)]
pub struct CyclicProcessBlueprint {
    /// The IDs of the possible output Resources of the CyclicProcess to create.
    pub output_resources_ids: Vec<i32>,

    /// The cycle interval of the CyclicProcess to create.
    pub cycle_interval: Duration,
}

impl CyclicProcessBlueprint {
    /// Creates a new [`CyclicProcessBlueprint`].
    pub fn new(
        output_resources_ids: Vec<i32>,
        cycle_interval: Duration,
    ) -> Result<Self, DomainError<ProcessErrorKind>> {
        if output_resources_ids.is_empty() {
            return Err(DomainError::from(ProcessErrorKind::NoOutputResources));
        }

        Ok(Self {
            output_resources_ids,
            cycle_interval,
        })
    }
}

/// Represents a [`output Resource assignment`][entity::cyclic_process_resources::ActiveModel] form for a CyclicProcess.
pub struct CyclicProcessOutputResourceAssignmentForm {
    /// The ID of the CyclicProcess.
    pub cyclic_process_id: i32,
    /// The ID of the output Resource.
    pub resource_id: i32,
}

impl CyclicProcessOutputResourceAssignmentForm {
    /// Creates a new [`CyclicProcessOutputResourceAssignmentForm`].
    pub fn new(cyclic_process_id: i32, resource_id: i32) -> Self {
        Self {
            cyclic_process_id,
            resource_id,
        }
    }
}

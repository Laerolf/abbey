use time::Duration;

use crate::features::output::forms::ResourceBlueprint;

/// Represents a Surroundings blueprint.
#[derive(Clone)]
pub struct SurroundingsBlueprint {
    pub source_blueprints: Vec<SurroundingsSourceBlueprint>,
}

impl SurroundingsBlueprint {
    /// Creates a new [`SurroundingsBlueprint`].
    pub fn new(source_blueprints: Vec<SurroundingsSourceBlueprint>) -> Self {
        Self { source_blueprints }
    }
}

/// Represents a blueprint for a Source in a Surroundings.
#[derive(Clone)]
pub struct SurroundingsSourceBlueprint {
    pub source_name: String,
    pub cyclic_process_blueprint: SurroundingsCyclicProcessSourceBlueprint,
}

impl SurroundingsSourceBlueprint {
    /// Creates a new [`SurroundingsSourceBlueprint`].
    pub fn new(
        source_name: impl Into<String>,
        cyclic_process_blueprint: SurroundingsCyclicProcessSourceBlueprint,
    ) -> Self {
        Self {
            source_name: source_name.into(),
            cyclic_process_blueprint,
        }
    }
}

/// Represents a blueprint for a CyclicProcess of a Source in a Surroundings.
#[derive(Clone)]
pub struct SurroundingsCyclicProcessSourceBlueprint {
    pub resource_blueprints: Vec<ResourceBlueprint>,
    pub cyclic_process_cycle_interval: Duration,
}

impl SurroundingsCyclicProcessSourceBlueprint {
    /// Creates a new [`SurroundingsCyclicProcessSourceBlueprint`].
    pub fn new(
        resource_blueprints: Vec<ResourceBlueprint>,
        cyclic_process_cycle_interval: Duration,
    ) -> Self {
        Self {
            resource_blueprints,
            cyclic_process_cycle_interval,
        }
    }
}

/// Represents a form assigning Sources to Surroundings.
#[derive(Clone)]
pub struct SurroundingSourceAssignmentForm {
    /// The ID of the Surroundings.
    pub surroundings_id: i32,
    /// The ID of the Source.
    pub source_id: i32,
}

impl SurroundingSourceAssignmentForm {
    /// Creates a new [`SurroundingSourceAssignmentForm`].
    pub fn new(surroundings_id: i32, source_id: i32) -> Self {
        Self {
            surroundings_id,
            source_id,
        }
    }
}

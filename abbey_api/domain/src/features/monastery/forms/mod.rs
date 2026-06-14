use crate::features::monk::forms::MonkBlueprint;

/// Represents a Monastery blueprint.
#[derive(Clone)]
pub struct MonasteryBlueprint {
    pub monk_blueprints: Vec<MonkBlueprint>,
}

impl MonasteryBlueprint {
    /// Creates a new [`MonasteryBlueprint`].
    pub fn new(monk_blueprints: Vec<MonkBlueprint>) -> Self {
        Self { monk_blueprints }
    }
}

/// Represents a form to assign a Monk to a Monastery.
pub struct MonasteryMonkAssignmentForm {
    /// The ID of the Monastery.
    pub monastery_id: i32,
    /// The ID of the Monk.
    pub monk_id: i32,
}

impl MonasteryMonkAssignmentForm {
    /// Creates a new [`MonasteryMonkAssignmentForm`].
    pub fn new(monastery_id: i32, monk_id: i32) -> Self {
        Self {
            monastery_id,
            monk_id,
        }
    }
}

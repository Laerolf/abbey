use crate::{
    features::{
        actor::error::ActorErrorKind, monastery::error::MonasteryErrorKind,
        monk::forms::MonkBlueprint,
    },
    shared::error::DomainError,
};

/// The default amount of Monks in a Monastery.
const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

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

    // TODO: Use the new method an generate monk blueprints with the game seed
    /// Creates a new [`MonasteryBlueprint`] with temporary default values.
    pub fn temp() -> Result<Self, DomainError<MonasteryErrorKind>> {
        let skill_names = vec!["cooking".to_string(), "brewing".to_string()];

        let monk_blueprints = (0..DEFAULT_AMOUNT_OF_MONKS)
            .map(|_| MonkBlueprint::new("Maurits", skill_names.clone()))
            .collect::<Result<Vec<MonkBlueprint>, DomainError<ActorErrorKind>>>()
            .map_err(|error| {
                DomainError::from(MonasteryErrorKind::MissingMonks).with_cause(error)
            })?;

        Ok(Self { monk_blueprints })
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

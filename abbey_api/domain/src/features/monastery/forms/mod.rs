use crate::{
    features::{
        game::domain::game_engine::GameEngine, monastery::error::MonasteryErrorKind,
        monk::forms::MonkBlueprint,
    },
    shared::error::DomainError,
};

/// The default amount of Monks in a Monastery.
const DEFAULT_AMOUNT_OF_MONKS: usize = 10;

/// Represents a Monastery blueprint.
#[derive(Clone)]
pub struct MonasteryBlueprint {
    pub monk_blueprints: Vec<MonkBlueprint>,
}

impl MonasteryBlueprint {
    /// Creates a new [`MonasteryBlueprint`].
    pub fn new(
        monk_blueprints: Vec<MonkBlueprint>,
    ) -> Result<Self, DomainError<MonasteryErrorKind>> {
        if monk_blueprints.is_empty() {
            return Err(DomainError::from(MonasteryErrorKind::MissingMonks));
        }

        Ok(Self { monk_blueprints })
    }

    /// Generates a new [`MonasteryBlueprint`].
    pub fn generate(
        monk_names: &[String],
        skill_names: &[String],
        game_engine: &mut GameEngine,
    ) -> Result<Self, DomainError<MonasteryErrorKind>> {
        if monk_names.is_empty() {
            return Err(DomainError::from(MonasteryErrorKind::Creation));
        }

        if skill_names.is_empty() {
            return Err(DomainError::from(MonasteryErrorKind::Creation));
        }

        let monk_blueprints = MonkBlueprint::generate_many(
            DEFAULT_AMOUNT_OF_MONKS,
            monk_names,
            skill_names,
            game_engine,
        )
        .map_err(|error| DomainError::from(MonasteryErrorKind::MissingMonks).with_cause(error))?;

        Self::new(monk_blueprints)
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

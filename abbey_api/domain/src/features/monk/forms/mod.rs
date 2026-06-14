use crate::{
    features::{actor::error::ActorErrorKind, game::domain::game_engine::GameEngine},
    shared::error::DomainError,
};

/// Represents a Monk blueprint.
#[derive(Clone)]
pub struct MonkBlueprint {
    /// The name of the Monk to create.
    pub name: String,
    /// The names of the Skills of the Monk to create.
    pub skill_names: Vec<String>,
}

impl MonkBlueprint {
    /// Creates a new [`MonkBlueprint`].
    pub fn new(
        name: impl Into<String>,
        skill_names: &[String],
    ) -> Result<Self, DomainError<ActorErrorKind>> {
        if skill_names.is_empty() {
            return Err(DomainError::from(ActorErrorKind::NoSkills));
        }

        Ok(Self {
            name: name.into(),
            skill_names: skill_names.iter().map(|name| name.to_string()).collect(),
        })
    }

    /// Generates the provided amount of [`MonkBlueprints`][Vec<MonkBlueprint>].
    pub fn generate_many(
        amount: usize,
        names: &[String],
        skill_names: &[String],
        game_engine: &mut GameEngine,
    ) -> Result<Vec<Self>, DomainError<ActorErrorKind>> {
        if names.is_empty() {
            return Err(DomainError::from(ActorErrorKind::Creation));
        }

        if skill_names.is_empty() {
            return Err(DomainError::from(ActorErrorKind::Creation));
        }

        (0..amount)
            .map(|_| {
                MonkBlueprint::new(
                    game_engine
                        .pick_random_element(names)
                        .ok_or_else(|| DomainError::from(ActorErrorKind::Creation))?,
                    skill_names,
                )
            })
            .collect::<Result<Vec<MonkBlueprint>, DomainError<ActorErrorKind>>>()
    }
}

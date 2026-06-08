use crate::{features::actor::error::ActorErrorKind, shared::error::DomainError};

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
        skill_names: Vec<impl Into<String>>,
    ) -> Result<Self, DomainError<ActorErrorKind>> {
        if skill_names.is_empty() {
            return Err(DomainError::from(ActorErrorKind::NoSkills));
        }

        Ok(Self {
            name: name.into(),
            skill_names: skill_names.into_iter().map(|name| name.into()).collect(),
        })
    }
}

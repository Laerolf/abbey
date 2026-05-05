use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{features::skill::domain::Skill, shared::DomainElement};

/// Represents a Skill.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SkillDto {
    /// The ID of the Skill.
    #[schema(example = 666)]
    pub id: i32,
    /// The name of the Skill.
    #[schema(example = "Brewing")]
    pub name: String,
}

impl SkillDto {
    /// Creates a [`SkillDto`] based on a [Skill].
    pub fn from(skill: Skill) -> Self {
        Self {
            id: skill.id().unwrap(),
            name: skill.name().to_string(),
        }
    }
}

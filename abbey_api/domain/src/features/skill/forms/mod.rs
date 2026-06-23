/// Represents a Skill blueprint.
#[derive(Clone)]
pub struct SkillBlueprint {
    /// The name of the Skill to create.
    pub name: String,
}

impl SkillBlueprint {
    /// Creates a new [`SkillBlueprint`].
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Represents a form to assign Skills to Monks.
pub struct MonkSkillAssignmentForm {
    /// The ID of the Monk.
    pub monk_id: i32,
    /// The ID of the Skill.
    pub skill_id: i32,
}

impl MonkSkillAssignmentForm {
    /// Creates a new [`MonkSkillAssignmentForm`].
    pub fn new(monk_id: i32, skill_id: i32) -> Self {
        Self { monk_id, skill_id }
    }
}

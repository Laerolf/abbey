/// Represents a [`Skill`][`entity::skills::ActiveModel`] creation form.
#[derive(Clone)]
pub struct SkillCreationForm {
    /// The name of this [`Skill`][`super::domain::Skill`].
    pub name: String,
}

impl SkillCreationForm {
    /// Creates a new [`SkillCreationForm`].
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Represents a [`Skill for a Monk`][entity::monk_skills::ActiveModel] creation form.
pub struct MonkSkillCreationForm {
    /// The ID of the Monk.
    pub monk_id: i32,
    /// The ID of the Skill.
    pub skill_id: i32,
}

impl MonkSkillCreationForm {
    /// Creates a new [`MonkSkillCreationForm`].
    pub fn new(monk_id: i32, skill_id: i32) -> Self {
        Self { monk_id, skill_id }
    }
}

/// Represents a [`Skill`][`super::domain::Skill`] creation form.
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

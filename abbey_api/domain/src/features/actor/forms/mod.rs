/// Represents a [`Monk`][`super::domain::monk`] creation form.
#[derive(Clone)]
pub struct MonkCreationForm {
    /// The name of this [`Monk`][`super::domain::monk`].
    pub name: String,
    /// The IDs of the [skills][`crate::features::skill::domain::Skill`] of this [`Monk`][`super::domain::monk`].
    pub skill_ids: Vec<i32>,
}

impl MonkCreationForm {
    /// Creates a new [`MonkCreationForm`].
    pub fn new(name: impl Into<String>, skill_ids: Vec<i32>) -> Self {
        Self {
            name: name.into(),
            skill_ids,
        }
    }
}

/// Represents a [`Monastery Monk`][entity::monastery_monks::Entity] creation form.
pub struct MonasteryMonkCreationForm {
    /// The ID of the Monastery.
    pub monastery_id: i32,
    /// The ID of the Monk.
    pub monk_id: i32,
}

impl MonasteryMonkCreationForm {
    /// Creates a new [`MonasteryMonkCreationForm`].
    pub fn new(monastery_id: i32, monk_id: i32) -> Self {
        Self {
            monastery_id,
            monk_id,
        }
    }
}

/// Represents a Monk creation form.
#[derive(Clone)]
pub struct MonkCreationForm {
    /// The name of the Monk to create.
    pub name: String,
    /// The names of the Skills of the Monk to create.
    pub skill_names: Vec<String>,
}

impl MonkCreationForm {
    /// Creates a new [`MonkCreationForm`].
    pub fn new(name: impl Into<String>, skill_names: Vec<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            skill_names: skill_names.into_iter().map(|name| name.into()).collect(),
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

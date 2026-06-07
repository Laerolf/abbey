use crate::features::monk::domain::Monk;

/// Represents a [Monastery][`crate::features::monastery::domain::Monastery`] creation form.
#[derive(Clone)]
pub struct MonasteryCreationForm {
    pub monks: Vec<Monk>,
}

impl MonasteryCreationForm {
    /// Creates a new [`MonasteryCreationForm`].
    pub fn new(monks: Vec<Monk>) -> Self {
        Self { monks }
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

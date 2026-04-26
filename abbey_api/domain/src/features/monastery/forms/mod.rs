use crate::features::actor::domain::monk::Monk;

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

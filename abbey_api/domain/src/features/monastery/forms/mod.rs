/// Represents a [Monastery][`crate::features::monastery::domain::Monastery`] creation form.
#[derive(Clone)]
pub struct MonasteryCreationForm {
    pub monk_ids: Vec<i32>,
}

impl MonasteryCreationForm {
    /// Creates a new [`MonasteryCreationForm`].
    pub fn new(monk_ids: Vec<i32>) -> Self {
        Self { monk_ids }
    }
}

/// Represents a [Surroundings][`super::domain::Surroundings`] creation form.
#[derive(Clone)]
pub struct SurroundingsCreationForm {
    /// The IDs of the [Source][`crate::features::source::domain::Source`]s of [Surroundings][`super::domain::Surroundings`].
    pub source_ids: Vec<i32>,
}

impl SurroundingsCreationForm {
    /// Creates a new [`SurroundingsCreationForm`].
    pub fn new(source_ids: Vec<i32>) -> Self {
        Self { source_ids }
    }
}

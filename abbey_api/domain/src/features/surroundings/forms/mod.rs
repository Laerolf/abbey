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

/// Represents a [`SurroundingsSource`][entity::surroundings_sources::ActiveModel] creation form.
pub struct SurroundingSourceCreationForm {
    /// The ID of the Surroundings.
    pub surroundings_id: i32,
    /// The ID of the Source.
    pub source_id: i32,
}

impl SurroundingSourceCreationForm {
    /// Creates a new [`SurroundingSourceCreationForm`].
    pub fn new(surroundings_id: i32, source_id: i32) -> Self {
        Self {
            surroundings_id,
            source_id,
        }
    }
}

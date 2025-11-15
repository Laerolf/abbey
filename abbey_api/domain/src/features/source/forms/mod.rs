/// Represents a [Source][`super::domain::Source`] creation form.
pub struct SourceCreationForm {
    /// The name of the [`Source`].
    pub name: String,

    /// The ID of the [Process][`crate::features::process::domain::CyclicProcess`] of this [Source][`super::domain::Source`].
    pub process_id: i32,
}

impl SourceCreationForm {
    /// Creates a new [`SourceCreationForm`].
    pub fn new(name: impl Into<String>, process_id: i32) -> Self {
        Self {
            name: name.into(),
            process_id,
        }
    }
}

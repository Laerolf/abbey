/// Represents a Source blueprint.
#[derive(Clone)]
pub struct SourceBlueprint {
    /// The name of the Source.
    pub name: String,
    /// The ID of the CyclicProcess of the Source.
    pub cyclic_process_id: i32,
}

impl SourceBlueprint {
    /// Creates a new [`SourceBlueprint`].
    pub fn new(name: impl Into<String>, cyclic_process_id: i32) -> Self {
        Self {
            name: name.into(),
            cyclic_process_id,
        }
    }
}

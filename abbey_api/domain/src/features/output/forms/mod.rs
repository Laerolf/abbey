use crate::features::output::domain::resource::Category;

/// Represents a Resource blueprint.
#[derive(Clone)]
pub struct ResourceBlueprint {
    /// The Name of the Resource.
    pub name: String,

    /// The category of the Resource.
    pub category: Category,
}

impl ResourceBlueprint {
    /// Creates a new [`ResourceBlueprint`].
    pub fn new(name: impl Into<String>, category: Category) -> Self {
        Self {
            name: name.into(),
            category,
        }
    }
}

/// Represents a [Resource][`super::domain::resource::Resource`] creation form.
pub struct ResourceCreationForm {
    /// The name of this [Resource][`super::domain::resource::Resource`] .
    pub name: String,

    /// The name of the [Category][`super::domain::resource::Category`] of this [Resource][`super::domain::resource::Resource`] .
    pub category_name: String,
}

impl ResourceCreationForm {
    /// Creates a new [`ResourceCreationForm`].
    pub fn new(name: impl Into<String>, category_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            category_name: category_name.into(),
        }
    }
}

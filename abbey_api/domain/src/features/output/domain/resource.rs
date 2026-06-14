use std::{fmt::Display, str::FromStr};

use time::OffsetDateTime;

use crate::{
    features::output::error::ResourceErrorKind,
    shared::{DomainElement, error::DomainError},
};

/// Represents a resource category.
#[derive(Debug, PartialEq, Clone)]
pub enum Category {
    /// A [Resource] that can be used to make things with.
    Material,
}

impl Category {
    /// Returns a string representing the [Category].
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Material => "material",
        }
    }
}

impl FromStr for Category {
    type Err = String;

    /// Returns the [Category] represented by the provided value.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "material" => Ok(Self::Material),
            _ => Err(format!("Invalid status: '{}'", s)),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
pub mod category_tests {
    use crate::features::output::domain::resource::Category;

    #[test]
    fn test_a_category_should_have_the_expected_string_value() {
        // Then
        assert_eq!("material", Category::Material.as_str())
    }
}

/// Represents a resource.
#[derive(PartialEq, Debug, Clone)]
pub struct Resource {
    /// The ID of this [Resource].
    id: Option<i32>,

    /// The creation date of this [`Resource`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Resource`].
    last_updated_at: Option<OffsetDateTime>,

    /// The name of this [Resource].
    name: String,

    /// The category of this [Resource].
    category: Category,
}

impl Resource {
    /// Creates a new [Resource].
    pub fn new(name: impl Into<String>, category: Category) -> Self {
        Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            name: name.into(),
            category,
        }
    }

    /// Creates a [Resource] based on the provided parameters.
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        name: impl Into<String>,
        category: Category,
    ) -> Self {
        Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            name: name.into(),
            category,
        }
    }

    /// Gets the name of this [`Resource`].
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Gets the category of this [`Resource`].
    pub fn category(&self) -> &Category {
        &self.category
    }
}

impl DomainElement<ResourceErrorKind> for Resource {
    /// Gets the ID of this [`Resource`].
    fn id(&self) -> Result<i32, DomainError<ResourceErrorKind>> {
        self.id
            .ok_or(DomainError::from(ResourceErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Resource`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Resource`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}

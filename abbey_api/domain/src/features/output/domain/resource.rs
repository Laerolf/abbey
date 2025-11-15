use std::{fmt::Display, str::FromStr};

/// Represents a resource category.
#[derive(Debug, PartialEq, Clone)]
pub enum Category {
    /// A [Resource] that can be used to make things with.
    Material,
}

impl Category {
    /// Returns a string representing the [Category].
    fn as_str(&self) -> &'static str {
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

/// Represents a resource.
#[derive(PartialEq, Debug, Clone)]
pub struct Resource {
    /// The ID of this [Resource].
    pub id: i32,

    /// The name of this [Resource].
    pub name: String,

    /// The category of this [Resource].
    pub category: Category,
}

impl Resource {
    /// Creates a new [Resource] based on the provided parameters.
    pub fn new(id: i32, name: impl Into<String>, category: Category) -> Self {
        Self {
            id,
            name: name.into(),
            category,
        }
    }
}

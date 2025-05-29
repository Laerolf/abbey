use std::fmt::Display;

use uuid::Uuid;

/// Represents a resource category.
#[derive(Debug, PartialEq, Clone)]
pub enum Category {
    /// A resource that can be used to make things with.
    Material,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Material => write!(f, "material"),
        }
    }
}

/// Represents a resource.
#[derive(PartialEq, Debug, Clone)]
pub struct Resource {
    /// The ID of this resource.
    pub id: Uuid,

    /// The name of this resource.
    pub name: String,

    /// The category of this resource.
    pub category: Category,
}

impl Resource {
    /// Creates a new resource based on the provided parameters.
    pub fn new(name: String, category: Category) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            category,
        }
    }
}

#[cfg(test)]
mod resource_tests {

    mod new_test {
        use crate::features::resource::{Category, Resource};

        #[test]
        fn a_new_resource_has_an_id() {
            // When
            let resource = Resource::new("Wood".into(), Category::Material);

            // Then
            assert!(!resource.id.to_string().is_empty());
        }

        #[test]
        fn a_new_resource_has_a_name() {
            // When
            let resource = Resource::new("Wood".into(), Category::Material);

            // Then
            assert!(!resource.name.is_empty());
        }

        #[test]
        fn a_new_resource_has_a_category() {
            // When
            let resource = Resource::new("Wood".into(), Category::Material);

            // Then
            assert_eq!("material", resource.category.to_string());
        }
    }
}

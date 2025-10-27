use resource::Resource;

pub mod resource;

/// Represents output, a [`resource::Resource`] and a quantity.
pub struct Output {
    /// The resource of the output.
    pub resource: Resource,

    /// The quantity of the output.
    pub quantity: i32,
}

impl Output {
    /// Creates a new output based on the provided resource and quantity.
    pub fn new(resource: Resource, quantity: i32) -> Self {
        Self { resource, quantity }
    }
}

#[cfg(test)]
mod output_tests {

    mod new_output {

        use crate::features::output::domain::{
            resource::{Category, Resource},
            Output,
        };

        #[test]
        fn a_new_output_has_a_resource() {
            // Given
            let resource = Resource::new("wood", Category::Material);

            // When
            let output = Output::new(resource.clone(), 10);

            // Then
            assert_eq!(resource, output.resource);
        }

        #[test]
        fn a_new_output_has_a_quantity() {
            // Given
            let resource = Resource::new("wood", Category::Material);

            // When
            let output = Output::new(resource, 10);

            // Then
            assert_eq!(10, output.quantity);
        }
    }
}

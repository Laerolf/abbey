use resource::Resource;

pub mod resource;

/// Represents output, a [`Resource`] and a quantity.
pub struct Output {
    /// The resource of the output.
    resource: Resource,

    /// The quantity of the output.
    quantity: i32,
}

impl Output {
    /// Creates a new output based on the provided resource and quantity.
    pub fn new(resource: Resource, quantity: i32) -> Self {
        Self { resource, quantity }
    }
}

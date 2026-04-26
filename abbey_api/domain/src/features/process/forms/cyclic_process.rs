use time::Duration;

/// Represents a [CyclicProcess][`super::domain::CyclicProcess`] creation form.
#[derive(Clone)]
pub struct CyclicProcessCreationForm {
    /// The IDs of the possible [Resources][`crate::features::output::domain::resource`] outputted by this [`CyclicProcess`][`super::domain::CyclicProcess`].
    pub output_resources_ids: Vec<i32>,

    /// The cycle interval of this [`CyclicProcess`][`super::domain::CyclicProcess`].
    pub cycle_interval: Duration,
}

impl CyclicProcessCreationForm {
    /// Creates a new [`CyclicProcessCreationForm`].
    pub fn new(output_resources_ids: Vec<i32>, cycle_interval: Duration) -> Self {
        Self {
            output_resources_ids,
            cycle_interval,
        }
    }
}

/// Represents a [`CyclicProcessResource`][entity::cyclic_process_resources::ActiveModel] creation form.
pub struct CyclicProcessResourceCreationForm {
    /// The ID of the Cyclic Process.
    pub cyclic_process_id: i32,
    /// The ID of the Resource.
    pub resource_id: i32,
}

impl CyclicProcessResourceCreationForm {
    /// Creates a new [`CyclicProcessResourceCreationForm`].
    pub fn new(cyclic_process_id: i32, resource_id: i32) -> Self {
        Self {
            cyclic_process_id,
            resource_id,
        }
    }
}

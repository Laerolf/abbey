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

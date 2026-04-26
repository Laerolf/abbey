use time::Duration;

/// Represents a [`Task`][`crate::features::process::domain::Task`] creation form.
#[derive(Clone)]
pub struct TaskCreationForm {
    /// The IDs of the input [Resources][`crate::features::output::domain::resource`] required by this [`Task`][`crate::features::process::domain::Task`].
    pub input_resources_ids: Vec<i32>,

    /// The IDs of the output [Resources][`crate::features::output::domain::resource`] of this [`Task`][`crate::features::process::domain::Task`].
    pub output_resources_ids: Vec<i32>,

    /// The duration of this [`Task`][`crate::features::process::domain::Task`].
    pub duration: Duration,
}

impl TaskCreationForm {
    /// Creates a new [`TaskCreationForm`].
    pub fn new(
        input_resources_ids: Vec<i32>,
        output_resources_ids: Vec<i32>,
        duration: Duration,
    ) -> Self {
        Self {
            input_resources_ids,
            output_resources_ids,
            duration,
        }
    }
}

/// Represents a [`TaskInputResource`][entity::task_input_resources::ActiveModel] creation form.
pub struct TaskInputResourceCreationForm {
    /// The ID of the Task.
    pub task_id: i32,
    /// The ID of the Resource.
    pub resource_id: i32,
}

impl TaskInputResourceCreationForm {
    /// Creates a new [`TaskInputResourceCreationForm`].
    pub fn new(task_id: i32, resource_id: i32) -> Self {
        Self {
            task_id,
            resource_id,
        }
    }
}

/// Represents a [`TaskOutputResource`][entity::task_output_resources::ActiveModel] creation form.
pub struct TaskOutputResourceCreationForm {
    /// The ID of the Task.
    pub task_id: i32,
    /// The ID of the Resource.
    pub resource_id: i32,
}

impl TaskOutputResourceCreationForm {
    /// Creates a new [`TaskOutputResourceCreationForm`].
    pub fn new(task_id: i32, resource_id: i32) -> Self {
        Self {
            task_id,
            resource_id,
        }
    }
}

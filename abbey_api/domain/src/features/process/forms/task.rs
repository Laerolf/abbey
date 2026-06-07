use time::Duration;

/// Represents a Task blueprint.
#[derive(Clone)]
pub struct TaskBlueprint {
    /// The IDs of the input Resource of the Task.
    pub input_resources_ids: Vec<i32>,

    /// The IDs of the output Resource of the Task.
    pub output_resources_ids: Vec<i32>,

    /// The duration of the Task.
    pub duration: Duration,
}

impl TaskBlueprint {
    /// Creates a new [`TaskBlueprint`].
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

/// Represents a form assigning input Resources to a Task.
pub struct TaskInputResourceAssignmentForm {
    /// The ID of the Task.
    pub task_id: i32,
    /// The ID of the Resource.
    pub resource_id: i32,
}

impl TaskInputResourceAssignmentForm {
    /// Creates a new [`TaskInputResourceAssignmentForm`].
    pub fn new(task_id: i32, resource_id: i32) -> Self {
        Self {
            task_id,
            resource_id,
        }
    }
}

/// Represents a form assigning input Resources to a Task.
pub struct TaskOutputResourceAssignmentForm {
    /// The ID of the Task.
    pub task_id: i32,
    /// The ID of the Resource.
    pub resource_id: i32,
}

impl TaskOutputResourceAssignmentForm {
    /// Creates a new [`TaskOutputResourceAssignmentForm`].
    pub fn new(task_id: i32, resource_id: i32) -> Self {
        Self {
            task_id,
            resource_id,
        }
    }
}

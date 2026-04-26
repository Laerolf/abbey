use crate::features::assignment::dto::ProcessAssignmentRequest;

/// Represents a form to assign a Process to an Actor, and the other way around.
#[derive(Clone)]
pub struct ProcessAssignmentForm {
    /// Indicates whether to assign the Player to the process.
    pub assign_player: bool,
    /// The IDs of the Actors to assign.
    pub actor_ids: Vec<i32>,
    /// The ID of the Process to assign.
    pub process_id: i32,
}

impl ProcessAssignmentForm {
    /// Creates a new [`ProcessAssignmentForm`] based on the provided parameters.
    pub fn new(assign_player: bool, actor_ids: Vec<i32>, process_id: i32) -> Self {
        Self {
            assign_player,
            actor_ids,
            process_id,
        }
    }

    /// Creates a [`ProcessAssignmentForm`] based on the provided [ProcessAssignmentRequest].
    pub fn from_request(request: ProcessAssignmentRequest) -> Self {
        Self {
            assign_player: request.assign_player,
            process_id: request.process_id,
            actor_ids: request.actor_ids,
        }
    }
}

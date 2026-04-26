use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::features::{
    actor::dto::ActorDto, assignment::domain::ProcessAssignment, process::dto::ProcessDto,
};

/// Represents a request to assign a Process to an Actor, and the other way around.
#[derive(Clone, Deserialize, ToSchema)]
pub struct ProcessAssignmentRequest {
    /// Indicates whether to assign the Player to the process.
    pub assign_player: bool,
    /// The IDs of the Actors to assign.
    pub actor_ids: Vec<i32>,
    /// The ID of the Process to assign.
    pub process_id: i32,
}

impl ProcessAssignmentRequest {
    /// Creates a [`ProcessAssignmentRequest`] based on the provided parameters.
    pub fn from(assign_player: bool, actor_ids: Vec<i32>, process_id: i32) -> Self {
        Self {
            assign_player,
            actor_ids,
            process_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "type")]
pub struct ProcessAssignmentDto {
    /// The [Actors][Vec<ActorDto>] of the [`ProcessAssignment`][ProcessAssignmentDto].
    pub actors: Vec<ActorDto>,
    pub process: ProcessDto,
}

impl ProcessAssignmentDto {
    /// Creates a [`ProcessAssignmentDto`] from a [ProcessAssignment].
    pub fn from(process_assignment: ProcessAssignment) -> Self {
        Self {
            actors: process_assignment
                .actors()
                .clone()
                .into_iter()
                .map(ActorDto::from)
                .collect(),
            process: ProcessDto::from(process_assignment.process().clone()),
        }
    }
}

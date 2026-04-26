use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use utoipa::ToSchema;

use crate::features::process::domain::{
    Process, ProcessKind, cyclic_process::CyclicProcess, task::Task,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "type")]
pub enum ProcessDto {
    /// The [CyclicProcess] [`ProcessDto`] variant.
    CyclicProcess(CyclicProcessDto),
    /// The [Task] [`ProcessDto`] variant.
    Task(TaskDto),
}

impl ProcessDto {
    /// Creates a [`ProcessDto`] from a [Process][ProcessKind].
    pub fn from(process: ProcessKind) -> Self {
        match process {
            ProcessKind::CyclicProcess(cyclic_process) => {
                ProcessDto::CyclicProcess(CyclicProcessDto::from(cyclic_process.clone()))
            }
            ProcessKind::Task(task) => ProcessDto::Task(TaskDto::from(task)),
        }
    }
}

/// Represents a [CyclicProcess] DTO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct CyclicProcessDto {
    /// The ID of the CyclicProcess.
    #[schema(example = 666)]
    pub id: i32,
    /// The status of the CyclicProcess.
    pub status: String,
    /// The time the CyclicProcess was started last.
    pub started_at: Option<OffsetDateTime>,
    /// The time the CyclicProcess was paused last.
    pub paused_at: Option<OffsetDateTime>,
    /// The cycle duration of the CyclicProcess.
    pub cycle_interval: Duration,
    /// The time that has elapsed since the CyclicProcess was started.
    pub elapsed: Duration,
}

impl CyclicProcessDto {
    /// Creates a [`CyclicProcessDto`] based on a [CyclicProcess].
    pub fn from(cyclic_process: CyclicProcess) -> Self {
        Self {
            id: cyclic_process.id().unwrap(),
            status: cyclic_process.status().to_string(),
            started_at: *cyclic_process.started_at(),
            paused_at: *cyclic_process.paused_at(),
            cycle_interval: *cyclic_process.cycle_interval(),
            elapsed: *cyclic_process.elapsed(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct TaskDto {
    /// The ID of the Task.
    #[schema(example = 666)]
    pub id: i32,
    /// The status of the Task.
    pub status: String,
    /// The time the Task was started last.
    pub started_at: Option<OffsetDateTime>,
    /// The time the Task was paused last.
    pub paused_at: Option<OffsetDateTime>,
    /// The duration of this Task.
    pub duration: Duration,
    /// The time that has elapsed since the Task was started.
    pub elapsed: Duration,
}

impl TaskDto {
    /// Creates a [`TaskDto`] based on a [Task].
    pub fn from(task: Task) -> Self {
        Self {
            id: task.id().unwrap(),
            status: task.status().to_string(),
            started_at: *task.started_at(),
            paused_at: *task.paused_at(),
            duration: *task.duration(),
            elapsed: *task.elapsed(),
        }
    }
}

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use utoipa::ToSchema;

use crate::{
    features::{
        actor::dto::ActorDto,
        process::domain::{
            Process, ProcessKind, Status, cyclic_process::CyclicProcess, task::Task,
        },
    },
    shared::{DomainElement, DurationDto},
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
    pub status: ProcessStatusDto,
    /// The time the CyclicProcess was started last.
    pub started_at: Option<String>,
    /// The time the CyclicProcess was paused last.
    pub paused_at: Option<String>,
    /// The cycle duration of the CyclicProcess.
    pub cycle_interval: DurationDto,
    /// The time that has elapsed since the CyclicProcess was started.
    pub elapsed: DurationDto,
    /// The assigned Actors to this CyclicProcess.
    pub assigned_actors: Vec<ActorDto>,
}

impl CyclicProcessDto {
    /// Creates a [`CyclicProcessDto`] based on a [CyclicProcess].
    pub fn from(cyclic_process: CyclicProcess) -> Self {
        let assigned_actors: Vec<ActorDto> = cyclic_process
            .assigned_actors()
            .clone()
            .into_iter()
            .map(ActorDto::from)
            .collect();

        Self {
            id: cyclic_process.id().unwrap(),
            status: ProcessStatusDto::from(*cyclic_process.status()),
            started_at: cyclic_process
                .clone()
                .started_at()
                .map(|timestamp| timestamp.format(&Rfc3339).unwrap()),
            paused_at: cyclic_process
                .clone()
                .paused_at()
                .map(|timestamp| timestamp.format(&Rfc3339).unwrap()),
            cycle_interval: DurationDto::from(*cyclic_process.cycle_interval()),
            elapsed: DurationDto::from(*cyclic_process.elapsed()),
            assigned_actors,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct TaskDto {
    /// The ID of the Task.
    #[schema(example = 666)]
    pub id: i32,
    /// The status of the Task.
    pub status: ProcessStatusDto,
    /// The time the Task was started last.
    pub started_at: Option<String>,
    /// The time the Task was paused last.
    pub paused_at: Option<String>,
    /// The duration of this Task.
    pub duration: DurationDto,
    /// The time that has elapsed since the Task was started.
    pub elapsed: DurationDto,
}

impl TaskDto {
    /// Creates a [`TaskDto`] based on a [Task].
    pub fn from(task: Task) -> Self {
        Self {
            id: task.id().unwrap(),
            status: ProcessStatusDto::from(*task.status()),
            started_at: task
                .clone()
                .started_at()
                .map(|timestamp| timestamp.format(&Rfc3339).unwrap()),
            paused_at: task
                .clone()
                .paused_at()
                .map(|timestamp| timestamp.format(&Rfc3339).unwrap()),
            duration: DurationDto::from(*task.duration()),
            elapsed: DurationDto::from(*task.elapsed()),
        }
    }
}

/// Represents the Status of a Process.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Copy, ToSchema)]
pub enum ProcessStatusDto {
    /// The Process has been created.
    New,
    /// The Process has started and is in progress.
    InProgress,
    /// The Process has been paused.
    Paused,
    /// The Process has been completed.
    Completed,
}

impl From<Status> for ProcessStatusDto {
    fn from(status: Status) -> Self {
        match status {
            Status::New => ProcessStatusDto::New,
            Status::InProgress => ProcessStatusDto::InProgress,
            Status::Paused => ProcessStatusDto::Paused,
            Status::Completed => ProcessStatusDto::Completed,
        }
    }
}

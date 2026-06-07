use entity::{task_input_resources, task_output_resources, tasks};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use std::str::FromStr;
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::domain::ActorKind,
        output::domain::resource::Resource,
        process::{
            domain::{
                Process as _, ProcessKind, Status,
                task::{Task, TaskState},
            },
            error::ProcessErrorKind,
            forms::task::{
                TaskBlueprint, TaskInputResourceAssignmentForm, TaskOutputResourceAssignmentForm,
            },
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that maps [`Task`] elements.
pub struct TaskMapper;

impl TaskMapper {
    /// Maps a [model][`tasks::Model`] to a [`Task`].
    pub fn to_domain_entity(
        model: tasks::Model,
        input_resources: Vec<Resource>,
        output_resources: Vec<Resource>,
        assigned_people: Vec<ActorKind>,
    ) -> Result<Task, DomainError<ProcessErrorKind>> {
        Task::restore(
            model.id,
            model.created_at,
            model.last_updated_at,
            TaskState {
                status: Status::from_str(&model.status)
                    .expect("Failed to find a process status with the provided value."),
                input_resources,
                output_resources,
                started_at: model.started_at,
                paused_at: model.paused_at,
                duration: Duration::seconds(model.duration.into()),
                elapsed: Duration::seconds(model.elapsed.into()),
                assigned_people,
            },
        )
    }

    /// Maps a [`TaskCreationForm`] to a [model][`tasks::ActiveModel`] to create.
    pub fn to_new_active_model(blueprint: TaskBlueprint) -> tasks::ActiveModel {
        let duration_in_seconds: i32 = blueprint
            .duration
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a task duration to seconds.");

        tasks::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            duration: Set(duration_in_seconds),
            status: Set(Status::New.to_string()),
            started_at: NotSet,
            paused_at: NotSet,
            elapsed: Set(0),
        }
    }

    /// Maps a [`Task`] to a [model][`tasks::ActiveModel`] to update.
    pub fn to_update_active_model(task: Task) -> tasks::ActiveModel {
        let duration_in_seconds: i32 = task
            .duration()
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a task's duration to seconds.");

        let elapsed_in_seconds: i32 = task
            .elapsed()
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a cyclic process elapsed to seconds.");

        tasks::ActiveModel {
            id: Unchanged(task.id().unwrap()),
            created_at: Unchanged(task.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            duration: Set(duration_in_seconds),
            status: Set(task.status().to_string()),
            started_at: Set(*task.started_at()),
            paused_at: Set(*task.paused_at()),
            elapsed: Set(elapsed_in_seconds),
        }
    }

    /// Maps a [Task model][`tasks::Model`] to a [`Process`][ProcessKind].
    pub fn to_process_kind(
        model: tasks::Model,
        input_resources: Vec<Resource>,
        output_resources: Vec<Resource>,
        assigned_people: Vec<ActorKind>,
    ) -> Result<ProcessKind, DomainError<ProcessErrorKind>> {
        Ok(ProcessKind::Task(TaskMapper::to_domain_entity(
            model,
            input_resources,
            output_resources,
            assigned_people,
        )?))
    }
}

/// Represents a mapper for [`TaskInputResource`][task_input_resources::Entity].
pub struct TaskInputResourceMapper;

impl TaskInputResourceMapper {
    /// Maps a [TaskInputResourceAssignmentForm] to a new [`model`][task_input_resources::ActiveModel].
    pub fn to_new_active_model(
        blueprint: TaskInputResourceAssignmentForm,
    ) -> task_input_resources::ActiveModel {
        task_input_resources::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            task_id: Set(blueprint.task_id),
            resource_id: Set(blueprint.resource_id),
        }
    }
}

/// Represents a mapper for [`TaskOutputResource`][task_output_resources::Entity].
pub struct TaskOutputResourceMapper;

impl TaskOutputResourceMapper {
    /// Maps a [TaskInputResourceAssignmentForm] to a new [`model`][task_output_resources::ActiveModel].
    pub fn to_new_active_model(
        blueprint: TaskOutputResourceAssignmentForm,
    ) -> task_output_resources::ActiveModel {
        task_output_resources::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            task_id: Set(blueprint.task_id),
            resource_id: Set(blueprint.resource_id),
        }
    }
}

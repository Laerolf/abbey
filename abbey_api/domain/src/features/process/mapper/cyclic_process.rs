use std::str::FromStr;

use entity::{cyclic_process_resources, cyclic_processes};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use time::{Duration, OffsetDateTime};

use crate::{
    features::{
        actor::domain::ActorKind,
        output::domain::resource::Resource,
        process::{
            domain::{
                Process, ProcessKind, Status,
                cyclic_process::{CyclicProcess, CyclicProcessState},
            },
            error::ProcessErrorKind,
            forms::cyclic_process::{CyclicProcessCreationForm, CyclicProcessResourceCreationForm},
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that maps [`CyclicProcess`] elements.
pub struct CyclicProcessMapper;

impl CyclicProcessMapper {
    /// Maps a [`CyclicProcessCreationForm`] to a [model][`cyclic_processes::ActiveModel`] to create.
    pub fn to_new_active_model(
        creation_form: CyclicProcessCreationForm,
    ) -> cyclic_processes::ActiveModel {
        let duration_in_seconds: i32 = creation_form
            .cycle_interval
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a cyclic process duration to seconds.");

        cyclic_processes::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            cycle_interval: Set(duration_in_seconds),
            status: Set(Status::New.to_string()),
            started_at: NotSet,
            paused_at: NotSet,
            elapsed: Set(0),
        }
    }

    /// Maps a [`CyclicProcess`] and a [`Resource`] to a [model][`cyclic_process_resources::ActiveModel`] to update.
    pub fn to_new_cyclic_process_resource_active_model(
        cyclic_process: &CyclicProcess,
        resource: &Resource,
    ) -> cyclic_process_resources::ActiveModel {
        cyclic_process_resources::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            cyclic_process_id: Set(cyclic_process.id().unwrap()),
            resource_id: Set(resource.id().unwrap()),
        }
    }

    /// Maps a [`CyclicProcess`] to a [model][`cyclic_processes::ActiveModel`] to update.
    pub fn to_update_active_model(cyclic_process: CyclicProcess) -> cyclic_processes::ActiveModel {
        let duration_in_seconds: i32 = cyclic_process
            .cycle_interval()
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a cyclic process duration to seconds.");

        let elapsed_in_seconds: i32 = cyclic_process
            .elapsed()
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a cyclic process elapsed to seconds.");

        cyclic_processes::ActiveModel {
            id: Unchanged(cyclic_process.id().unwrap()),
            created_at: Unchanged(cyclic_process.created_at().unwrap()),
            last_updated_at: Set(Some(OffsetDateTime::now_utc())),
            cycle_interval: Set(duration_in_seconds),
            status: Set(cyclic_process.status().to_string()),
            started_at: Set(*cyclic_process.started_at()),
            paused_at: Set(*cyclic_process.paused_at()),
            elapsed: Set(elapsed_in_seconds),
        }
    }

    /// Maps a [model][`cyclic_processes::Model`] to a [`CyclicProcess`].
    pub fn to_domain_entity(
        model: cyclic_processes::Model,
        output_resources: Vec<Resource>,
        assigned_actors: Vec<ActorKind>,
    ) -> Result<CyclicProcess, DomainError<ProcessErrorKind>> {
        CyclicProcess::restore(
            model.id,
            model.created_at,
            model.last_updated_at,
            CyclicProcessState {
                status: Status::from_str(&model.status)
                    .expect("Failed to find a process status with the provided value."),
                output_resources,
                started_at: model.started_at,
                paused_at: model.paused_at,
                cycle_interval: Duration::seconds(model.cycle_interval.into()),
                elapsed: Duration::seconds(model.elapsed.into()),
                assigned_people: assigned_actors,
            },
        )
    }

    /// Maps a [CyclicProcess models][`cyclic_processes::Model`] to a [`Process`][ProcessKind].
    pub fn to_process_kind(
        model: cyclic_processes::Model,
        output_resources: Vec<Resource>,
        assigned_people: Vec<ActorKind>,
    ) -> Result<ProcessKind, DomainError<ProcessErrorKind>> {
        Ok(ProcessKind::CyclicProcess(
            CyclicProcessMapper::to_domain_entity(model, output_resources, assigned_people)?,
        ))
    }
}

/// Represents a mapper for [`CyclicProcessResource`][cyclic_process_resources::Entity].
pub struct CyclicProcessResourceMapper;

impl CyclicProcessResourceMapper {
    /// Maps a [CyclicProcessResourceCreationForm] to a new [`model`][cyclic_process_resources::ActiveModel].
    pub fn to_new_active_model(
        creation_form: CyclicProcessResourceCreationForm,
    ) -> cyclic_process_resources::ActiveModel {
        cyclic_process_resources::ActiveModel {
            id: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            last_updated_at: NotSet,
            cyclic_process_id: Set(creation_form.cyclic_process_id),
            resource_id: Set(creation_form.resource_id),
        }
    }
}

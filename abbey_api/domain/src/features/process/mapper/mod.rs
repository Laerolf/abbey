use std::{cell::RefCell, rc::Rc, str::FromStr};

use entity::{cyclic_process_resources, cyclic_processes};
use sea_orm::ActiveValue::{NotSet, Set};
use time::Duration;

use crate::features::{
    actor::domain::person::Person,
    output::domain::resource::Resource,
    process::{
        domain::{CyclicProcess, Status},
        forms::CyclicProcessCreationForm,
    },
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
            cycle_interval: Set(duration_in_seconds),
            status: Set(Status::New.to_string()),
            started_at: NotSet,
            paused_at: NotSet,
            elapsed: NotSet,
        }
    }

    /// Maps a [`CyclicProcess`] and a [`Resource`] to a [model][`cyclic_process_resources::ActiveModel`] to update.
    pub fn to_new_cyclic_process_resource_active_model(
        cyclic_process: &CyclicProcess,
        resource: &Resource,
    ) -> cyclic_process_resources::ActiveModel {
        cyclic_process_resources::ActiveModel {
            id: NotSet,
            cylic_process_id: Set(cyclic_process.id),
            resource_id: Set(resource.id),
        }
    }

    /// Maps a [`CyclicProcess`] to a [model][`cyclic_processes::ActiveModel`] to update.
    pub fn to_update_active_model(cyclic_process: &CyclicProcess) -> cyclic_processes::ActiveModel {
        let duration_in_seconds: i32 = cyclic_process
            .cycle_interval
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a cyclic process duration to seconds.");

        let elapsed_in_seconds: i32 = cyclic_process
            .elapsed
            .whole_seconds()
            .try_into()
            .map_err(|_| "The duration is too large to fit in i32.")
            .expect("Failed to convert a cyclic process elapsed to seconds.");

        cyclic_processes::ActiveModel {
            id: Set(cyclic_process.id),
            cycle_interval: Set(duration_in_seconds),
            status: Set(cyclic_process.status.to_string()),
            started_at: Set(cyclic_process.started_at),
            paused_at: Set(cyclic_process.paused_at),
            elapsed: Set(elapsed_in_seconds),
        }
    }

    /// Maps a [model][`cyclic_processes::Model`] to a [`CyclicProcess`].
    pub fn to_domain_entity(
        model: cyclic_processes::Model,
        output_resources: Vec<Resource>,
        assigned_people: Vec<Rc<RefCell<dyn Person>>>,
    ) -> CyclicProcess {
        CyclicProcess::new(
            model.id,
            Status::from_str(&model.status)
                .expect("Failed to find a process status with the provided value."),
            output_resources,
            model.started_at,
            model.paused_at,
            Duration::seconds(model.cycle_interval.into()),
            Duration::seconds(model.elapsed.into()),
            assigned_people,
        )
    }
}

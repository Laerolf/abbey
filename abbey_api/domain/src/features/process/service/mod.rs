use std::{cell::RefCell, rc::Rc};

use entity::cyclic_process_resources;
use futures::future::try_join_all;
use sea_orm::DbErr;
use time::Duration;

use crate::{
    features::{
        actor::domain::person::Person,
        output::domain::resource::Resource,
        process::{
            domain::CyclicProcess,
            error::ProcessError,
            forms::CyclicProcessCreationForm,
            mapper::CyclicProcessMapper,
            repository::{CyclicProcessRepository, CyclicProcessResourceRepository},
        },
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`CyclicProcess`] topic.
#[derive(Default)]
pub struct CyclicProcessService {
    repository: CyclicProcessRepository,
    cyclic_process_resource_repository: CyclicProcessResourceRepository,
}

impl CyclicProcessService {
    async fn add_resource_to_cyclic_process(
        &self,
        cyclic_process: &CyclicProcess,
        resource: &Resource,
    ) -> Result<cyclic_process_resources::Model, DbErr> {
        self.cyclic_process_resource_repository
            .insert(
                CyclicProcessMapper::to_new_cyclic_process_resource_active_model(
                    cyclic_process,
                    resource,
                ),
            )
            .await
    }

    /// Creates a new [`CyclicProcess`].
    pub async fn create_cyclic_process(
        &self,
        output_resources: Vec<Resource>,
        cycle_interval: Duration,
        assigned_people: Vec<Rc<RefCell<dyn Person>>>,
    ) -> Result<CyclicProcess, Box<dyn DomainError>> {
        let creation_form = CyclicProcessCreationForm::new(
            output_resources
                .iter()
                .map(|resource| resource.id)
                .collect(),
            cycle_interval,
        );

        match self
            .repository
            .insert(CyclicProcessMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_cyclic_process) => {
                let process = CyclicProcessMapper::to_domain_entity(
                    new_cyclic_process,
                    output_resources.clone(),
                    assigned_people,
                );

                match try_join_all(output_resources.iter().map(|output_resource| {
                    self.add_resource_to_cyclic_process(&process, output_resource)
                }))
                .await
                {
                    Ok(_) => Ok(process),
                    Err(_) => Err(Box::new(ProcessError::Creation)),
                }
            }
            Err(_error) => Err(Box::new(ProcessError::Creation)),
        }
    }
}

use std::sync::{Arc, Mutex};

use crate::{
    features::{
        actor::domain::person::Person, assignment::error::AssignmentErrorKind,
        process::domain::Process,
    },
    shared::error::DomainErrorKind,
};

pub struct ProcessAssignmentFactory {}

impl ProcessAssignmentFactory {
    /// Assigns a [Person][`crate::features::actor::domain::person`] to a [Process][`crate::features::process::domain::Process`].
    pub fn assign_process_to_person(
        person: Arc<Mutex<dyn Person>>,
        process: Arc<Mutex<dyn Process>>,
    ) -> Result<(), Box<dyn DomainErrorKind>> {
        let mut mut_person = person.lock().unwrap();

        if mut_person.assign_process(process.clone()).is_err() {
            return Err(Box::new(AssignmentErrorKind::ActorAssigned));
        }

        process.lock().unwrap().assign_person(person.clone());

        Ok(())
    }

    /// Unassigns a [Person][`crate::features::actor::domain::person`] from a [Process][`crate::features::process::domain::Process`].
    pub fn unassign_process_from_person(
        person: Arc<Mutex<dyn Person>>,
        process: Arc<Mutex<dyn Process>>,
    ) {
        let mut mut_person = person.lock().unwrap();
        let mut mut_process = process.lock().unwrap();

        mut_person.unassign_process();
        mut_process.unassign_person(&person);
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::{
    features::{
        actor::domain::person::Person, assignment::error::AssignmentError, process::domain::Process,
    },
    shared::error::DomainError,
};

pub struct ProcessAssignmentFactory {}

impl ProcessAssignmentFactory {
    /// Assigns a [Person][`crate::features::actor::domain::person`] to a [Process][`crate::features::process::domain::Process`].
    pub fn assign_process_to_person(
        person: Rc<RefCell<dyn Person>>,
        process: Rc<RefCell<dyn Process>>,
    ) -> Result<(), Box<dyn DomainError>> {
        let mut mut_person = person.borrow_mut();
        let mut mut_process = process.borrow_mut();

        if mut_person.assign_process(Rc::clone(&process)).is_err() {
            return Err(Box::new(AssignmentError::ActorAssigned));
        }

        mut_process.assign_person(Rc::clone(&person));

        Ok(())
    }

    /// Unassigns a [Person][`crate::features::actor::domain::person`] from a [Process][`crate::features::process::domain::Process`].
    pub fn unassign_process_from_person(
        person: Rc<RefCell<dyn Person>>,
        process: Rc<RefCell<dyn Process>>,
    ) {
        let mut mut_person = person.borrow_mut();
        let mut mut_process = process.borrow_mut();

        mut_person.unassign_process();
        mut_process.unassign_person(&Rc::clone(&person));
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::features::{actor::domain::person::Person, process::domain::Process};

pub struct ProcessAssignmentFactory {}

impl ProcessAssignmentFactory {
    pub fn assign_process_to_person(
        person: Rc<RefCell<dyn Person>>,
        process: Rc<RefCell<dyn Process>>,
    ) {
        let mut mut_person = person.borrow_mut();
        let mut mut_process = process.borrow_mut();

        mut_person.assign_process(Rc::clone(&process));
        mut_process.assign_person(Rc::clone(&person));
    }

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

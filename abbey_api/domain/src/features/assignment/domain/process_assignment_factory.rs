use crate::{
    features::{
        actor::domain::{Actor, ActorKind},
        assignment::{domain::ProcessAssignment, error::AssignmentErrorKind},
        process::domain::{Process, ProcessKind},
    },
    shared::error::DomainError,
};

pub struct ProcessAssignmentFactory;

impl ProcessAssignmentFactory {
    /// Assigns [Actors][Vec<ActorKind>] to a [`Process`][ProcessKind].
    pub fn assign_process_to_actors(
        actors: Vec<ActorKind>,
        mut process: ProcessKind,
    ) -> Result<ProcessAssignment, DomainError<AssignmentErrorKind>> {
        let updated_actors = actors
            .into_iter()
            .map(|mut actor| {
                actor.assign_process(process.clone()).map_err(|error| {
                    DomainError::from(AssignmentErrorKind::ActorAssigned).with_cause(error)
                })?;
                process.assign_person(actor.clone());

                Ok(actor)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ProcessAssignment::from(updated_actors, process))
    }

    /// Unassigns [Actors][Vec<ActorKind>] from a [`Process`][ProcessKind].
    pub fn unassign_process_from_people(mut actors: Vec<ActorKind>, mut process: ProcessKind) {
        actors.iter_mut().for_each(|actor| {
            actor.unassign_process();
            process.unassign_person(actor);
        });
    }
}

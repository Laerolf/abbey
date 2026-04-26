use crate::features::{actor::domain::ActorKind, process::domain::ProcessKind};

pub mod process_assignment_factory;

#[derive(Debug)]
pub struct ProcessAssignment {
    /// The [Actors][Vec<ActorKind>] of the [`ProcessAssignment`].
    actors: Vec<ActorKind>,
    /// The [Process][ProcessKind] of the [`ProcessAssignment`].
    process: ProcessKind,
}

impl ProcessAssignment {
    /// Creates a [`ProcessAssignment`].
    pub fn from(actors: Vec<ActorKind>, process: ProcessKind) -> Self {
        Self { actors, process }
    }

    pub fn actors(&self) -> &Vec<ActorKind> {
        &self.actors
    }

    pub fn process(&self) -> &ProcessKind {
        &self.process
    }
}

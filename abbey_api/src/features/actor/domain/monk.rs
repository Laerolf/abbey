use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    features::{
        actor::{
            domain::{actor_status::ActorStatus, person::Person, Actor},
            error::ActorError,
        },
        process::domain::Process,
        skill::domain::Skill,
    },
    shared::error::DomainError,
};

/// Represents a monk.
pub struct Monk {
    /// The skills of this monk.
    skills: Vec<Skill>,

    /// The assigned task of this monk.
    assigned_process: Option<Weak<RefCell<dyn Process>>>,
}

impl Monk {
    /// Creates a new [`monk::Monk`]
    pub fn new() -> Self {
        Self {
            skills: Vec::new(),
            assigned_process: None,
        }
    }
}

impl Person for Monk {
    /// Returns the skill set of this monk.
    fn skills(&self) -> &Vec<Skill> {
        &self.skills
    }
}

impl Actor for Monk {
    /// Gets the status of this monk.
    fn status(&self) -> ActorStatus {
        if self.assigned_process.is_some() {
            return ActorStatus::Assigned;
        }

        ActorStatus::Available
    }

    /// Assigns a [`crate::features::process::Process`] to this monk.
    fn assign_process(
        &mut self,
        process: Rc<RefCell<dyn Process>>,
    ) -> Result<(), Box<dyn DomainError>> {
        if self.status() == ActorStatus::Assigned {
            return Err(Box::new(ActorError::Assigned));
        }

        self.assigned_process = Some(Rc::downgrade(&process));

        Ok(())
    }

    /// Unassigns a [`crate::features::process::Process`] from this monk.
    fn unassign_process(&mut self) {
        self.assigned_process = None;
    }
}

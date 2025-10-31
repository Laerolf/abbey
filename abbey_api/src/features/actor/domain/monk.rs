use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use uuid::Uuid;

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
    /// The [ID][`uuid::Uuid`] of this [`Monk`].
    pub id: Uuid,

    /// The name of this [`Monk`].
    pub name: String,

    /// The [skills][`crate::features::skill::domain::Skill`] of this [`Monk`].
    skills: Vec<Skill>,

    /// The assigned [process][`crate::features::process::domain::Process`] of this [`Monk`].
    assigned_process: Option<Weak<RefCell<dyn Process>>>,
}

impl Monk {
    /// Creates a new [`Monk`]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            skills: Vec::new(),
            assigned_process: None,
        }
    }
}

impl Person for Monk {
    /// Returns the [skill set][`crate::features::skill::domain::Skill`] of this [`Monk`].
    fn skills(&self) -> &Vec<Skill> {
        &self.skills
    }
}

impl Actor for Monk {
    /// Gets the [status][`crate::features::actor::domain::ActorStatus`] of this [`Monk`].
    fn status(&self) -> ActorStatus {
        if self.assigned_process.is_some() {
            return ActorStatus::Assigned;
        }

        ActorStatus::Available
    }

    /// Assigns a [process][`crate::features::process::domain::Process`] to this [`Monk`].
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

    /// Unassigns a [process][`crate::features::process::domain::Process`] from this [`Monk`].
    fn unassign_process(&mut self) {
        self.assigned_process = None;
    }
}

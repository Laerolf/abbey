use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    features::{
        actor::{
            domain::{Actor, actor_status::ActorStatus, person::Person},
            error::ActorError,
        },
        process::domain::Process,
        skill::domain::Skill,
    },
    shared::error::DomainError,
};

/// Represents a monk.
#[derive(Clone)]
pub struct Monk {
    /// The ID of this [`Monk`].
    pub id: i32,

    /// The name of this [`Monk`].
    pub name: String,

    /// The [skills][`crate::features::skill::domain::Skill`] of this [`Monk`].
    pub skills: Vec<Skill>,

    /// The assigned [process][`crate::features::process::domain::Process`] of this [`Monk`].
    pub assigned_process: Option<Weak<RefCell<dyn Process>>>,
}

impl Monk {
    /// Creates a new [`Monk`]
    pub fn new(
        id: i32,
        name: impl Into<String>,
        skills: Vec<Skill>,
        assigned_process: Option<Weak<RefCell<dyn Process>>>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            skills,
            assigned_process,
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

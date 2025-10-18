use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::features::{
    actor::domain::{person::Person, Actor},
    process::domain::Process,
    skill::domain::Skill,
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
    /// Assigns a [`crate::features::process::Process`] to this monk.
    fn assign_process(&mut self, process: Rc<RefCell<dyn Process>>) {
        self.assigned_process = Some(Rc::downgrade(&process));
    }

    /// Unassigns a [`crate::features::process::Process`] from this monk.
    fn unassign_process(&mut self) {
        self.assigned_process = None;
    }
}

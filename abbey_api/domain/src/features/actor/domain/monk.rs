use crate::{
    features::{
        actor::{
            domain::{Actor, actor_status::ActorStatus, person::Person},
            error::ActorErrorKind,
        },
        process::domain::ProcessKind,
        skill::domain::Skill,
    },
    shared::error::DomainError,
};

/// Represents a monk.
#[derive(Clone, Debug)]
pub struct Monk {
    /// The ID of this [`Monk`].
    id: Option<i32>,

    /// The name of this [`Monk`].
    name: String,

    /// The [skills][`crate::features::skill::domain::Skill`] of this [`Monk`].
    skills: Vec<Skill>,

    /// The assigned [process][`ProcessKind`] of this [`Monk`].
    assigned_process: Option<ProcessKind>,
}

impl Monk {
    /// Creates a new [`Monk`]
    pub fn new(
        name: impl Into<String>,
        skills: Vec<Skill>,
        assigned_process: Option<ProcessKind>,
    ) -> Result<Self, DomainError<ActorErrorKind>> {
        if skills.is_empty() {
            return Err(DomainError::from(ActorErrorKind::NoSkills));
        }

        Ok(Self {
            id: None,
            name: name.into(),
            skills,
            assigned_process,
        })
    }

    /// Creates a [`Monk`]
    pub fn restore(
        id: i32,
        name: impl Into<String>,
        skills: Vec<Skill>,
        assigned_process: Option<ProcessKind>,
    ) -> Result<Self, DomainError<ActorErrorKind>> {
        if skills.is_empty() {
            return Err(DomainError::from(ActorErrorKind::NoSkills));
        }

        Ok(Self {
            id: Some(id),
            name: name.into(),
            skills,
            assigned_process,
        })
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Person for Monk {
    /// Returns the [skill set][`crate::features::skill::domain::Skill`] of this [`Monk`].
    fn skills(&self) -> &Vec<Skill> {
        &self.skills
    }
}

impl Actor for Monk {
    /// Gets the ID of the [`Monk`].
    fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Gets the [status][`ActorStatus`] of this [`Monk`].
    fn status(&self) -> &ActorStatus {
        if self.assigned_process.is_some() {
            return &ActorStatus::Assigned;
        }

        &ActorStatus::Available
    }

    /// Gets the [assigned process][`ProcessKind`] of this [`Monk`].
    fn assigned_process(&self) -> &Option<ProcessKind> {
        &self.assigned_process
    }

    /// Assigns a [process][`ProcessKind`] to this [`Monk`].
    fn assign_process(&mut self, process: ProcessKind) -> Result<(), DomainError<ActorErrorKind>> {
        if *self.status() == ActorStatus::Assigned {
            return Err(DomainError::from(ActorErrorKind::Assigned));
        }

        self.assigned_process = Some(process);

        Ok(())
    }

    /// Unassigns a [process][`crate::features::process::domain::Process`] from this [`Monk`].
    fn unassign_process(&mut self) {
        self.assigned_process = None;
    }
}

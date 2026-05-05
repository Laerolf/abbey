use time::OffsetDateTime;

use crate::{
    features::{
        actor::{
            domain::{Actor, actor_status::ActorStatus, person::Person},
            error::ActorErrorKind,
        },
        process::domain::ProcessKind,
        skill::domain::Skill,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a monk.
#[derive(Clone, Debug)]
pub struct Monk {
    /// The ID of this [`Monk`].
    id: Option<i32>,

    /// The creation date of this [`Monk`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Monk`].
    last_updated_at: Option<OffsetDateTime>,

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
            created_at: None,
            last_updated_at: None,
            name: name.into(),
            skills,
            assigned_process,
        })
    }

    /// Creates a [`Monk`]
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        name: impl Into<String>,
        skills: Vec<Skill>,
        assigned_process: Option<ProcessKind>,
    ) -> Result<Self, DomainError<ActorErrorKind>> {
        if skills.is_empty() {
            return Err(DomainError::from(ActorErrorKind::NoSkills));
        }

        Ok(Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
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

impl DomainElement<ActorErrorKind> for Monk {
    /// Gets the ID of this [`Monk`].
    fn id(&self) -> Result<i32, DomainError<ActorErrorKind>> {
        self.id
            .ok_or(DomainError::from(ActorErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Monk`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Monk`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}

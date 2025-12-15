use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum SkillErrorKind {
    /// Failed to create a new [`Skill`][`super::domain::Skill`].
    Creation,
}

impl DomainErrorKind for SkillErrorKind {
    /// Gets the locale code of the [`SkillError`].
    fn code(&self) -> String {
        match self {
            Self::Creation => "error.skill.creation".to_string(),
        }
    }

    /// Gets the message of the [`SkillError`].
    fn message(&self) -> String {
        match self {
            Self::Creation => "Failed to create a new Skill.".to_string(),
        }
    }
}

impl Display for SkillErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

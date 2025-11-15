use std::fmt::Display;

use crate::shared::error::DomainError;

#[derive(Debug)]
pub enum SkillError {
    /// Failed to create a new [`Skill`][`super::domain::Skill`].
    Creation,
}

impl std::error::Error for SkillError {}

impl DomainError for SkillError {
    /// Gets the locale code of the [`SkillError`].
    fn code(&self) -> &'static str {
        match self {
            Self::Creation => "error.skill.creation",
        }
    }

    /// Gets the message of the [`SkillError`].
    fn message(&self) -> &'static str {
        match self {
            Self::Creation => "Failed to create a new Skill.",
        }
    }
}

impl Display for SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

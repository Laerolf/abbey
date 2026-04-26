use crate::features::skill::domain::Skill;

/// Represent a person.
pub trait Person {
    /// Returns the [skill set][Vec<Skill>] of a [`Person`].
    fn skills(&self) -> &Vec<Skill>;
}

use crate::features::{actor::domain::Actor, skill::domain::Skill};

/// Represent a person.
pub trait Person: Actor {
    /// Returns the skill set of a person.
    fn skills(&self) -> &Vec<Skill>;
}

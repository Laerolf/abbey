use crate::features::{actor::domain::Actor, skill::domain::Skill};

/// Represent a person.
pub trait Person: Actor {
    /// Returns the ID of a [`Person`].
    fn id(&self) -> i32;

    /// Returns the [skill set][`crate::features::skill::domain::Skill`] of a [`Person`].
    fn skills(&self) -> &Vec<Skill>;
}

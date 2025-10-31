use uuid::Uuid;

/// Represents a skill.
pub struct Skill {
    /// The ID of this [`Skill`].
    pub id: Uuid,

    /// The name of this [`Skill`].
    pub name: String,
}

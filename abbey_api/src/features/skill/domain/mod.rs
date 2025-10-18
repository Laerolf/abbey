use uuid::Uuid;

/// Represents a skill.
pub struct Skill {
    /// The ID of this skill.
    pub id: Uuid,

    /// The name of this skill.
    pub name: String,
}

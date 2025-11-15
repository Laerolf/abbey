/// Represents a skill.
#[derive(Clone)]
pub struct Skill {
    /// The ID of this [`Skill`].
    pub id: i32,

    /// The name of this [`Skill`].
    pub name: String,
}

impl Skill {
    /// Creates a new [`Skill`].
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}

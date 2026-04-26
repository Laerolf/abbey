/// Represents a skill.
#[derive(Clone, Debug)]
pub struct Skill {
    /// The ID of this [`Skill`].
    id: Option<i32>,

    /// The name of this [`Skill`].
    name: String,
}

impl Skill {
    /// Creates a new [`Skill`].
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
        }
    }

    /// Creates a [`Skill`].
    pub fn restore(id: i32, name: impl Into<String>) -> Self {
        Self {
            id: Some(id),
            name: name.into(),
        }
    }

    /// Gets the ID of this [`Skill`].
    pub fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Gets the name of this [`Skill`].
    pub fn name(&self) -> &String {
        &self.name
    }
}

/// Represents a Monk blueprint.
#[derive(Clone)]
pub struct MonkBlueprint {
    /// The name of the Monk to create.
    pub name: String,
    /// The names of the Skills of the Monk to create.
    pub skill_names: Vec<String>,
}

impl MonkBlueprint {
    /// Creates a new [`MonkBlueprint`].
    pub fn new(name: impl Into<String>, skill_names: Vec<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            skill_names: skill_names.into_iter().map(|name| name.into()).collect(),
        }
    }
}

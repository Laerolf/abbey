use crate::features::source::domain::Source;

/// Represents the surroundings of a [Monastery][`crate::features::monastery::domain::Monastery`].
pub struct Surroundings {
    /// The ID of this [`Surroundings`].
    pub id: Uuid,

    /// The [Sources][`crate::features::source::domain::Source`] belonging to this [`Surroundings`].
    pub sources: Vec<Source>,
}

impl Surroundings {
    /// Creates a new [`Surroundings`].
    pub fn new(sources: Vec<Source>) -> Self {
        Self {
            id: Uuid::new_v4(),
            sources,
        }
    }
}

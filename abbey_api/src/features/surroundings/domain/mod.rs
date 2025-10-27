use crate::features::source::domain::Source;

pub struct Surroundings {
    /// The sources belonging to this surroundings.
    pub sources: Vec<Source>,
}

impl Surroundings {
    pub fn new(sources: Vec<Source>) -> Self {
        Self { sources }
    }
}

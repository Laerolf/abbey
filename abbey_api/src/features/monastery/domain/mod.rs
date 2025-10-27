use crate::features::actor::domain::monk::Monk;

pub struct Monastery {
    /// The monks in a monastery.
    pub monks: Vec<Monk>,
}

impl Monastery {
    /// Creates a new Monastery.
    pub fn new(monks: Vec<Monk>) -> Self {
        Self { monks }
    }
}

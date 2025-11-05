use crate::features::actor::domain::monk::Monk;

pub struct Monastery {
    /// The ID of this [`Monastery`].
    pub id: Uuid,

    /// The [Monks][`crate::features::actor::domain::monk`] in a [`Monastery`].
    pub monks: Vec<Monk>,
}

impl Monastery {
    /// Creates a new [`Monastery`].
    pub fn new(monks: Vec<Monk>) -> Self {
        Self {
            id: Uuid::new_v4(),
            monks,
        }
    }
}

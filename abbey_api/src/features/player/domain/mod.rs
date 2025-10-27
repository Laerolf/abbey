use uuid::Uuid;

pub struct Player {
    /// The ID of this Player.
    pub id: Uuid,
}

impl Player {
    /// Creates a new Player.
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

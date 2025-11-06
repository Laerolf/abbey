/// Represents a [Game][`crate::features::game::domain::Game`] creation form.
pub struct GameCreationForm {
    /// The ID of the [Player][`crate::features::player::domain::Player`] of the [Game][`crate::features::game::domain::Game`] to create.
    pub player_id: i32,

    /// The ID of the [Monastery][`crate::features::monastery::domain::Monastery`] of the [Game][`crate::features::game::domain::Game`] to create.
    pub monastery_id: i32,

    /// The ID of the [Surroundings][`crate::features::surroundings::domain::Surroundings`] of the [Monastery][`crate::features::monastery::domain::Monastery`] in the [Game][`crate::features::game::domain::Game`] to create.
    pub surroundings_id: i32,
}

impl GameCreationForm {
    /// Creates a new [`GameCreationForm`].
    pub fn new(player_id: i32, monastery_id: i32, surroundings_id: i32) -> Self {
        Self {
            player_id,
            monastery_id,
            surroundings_id,
        }
    }
}

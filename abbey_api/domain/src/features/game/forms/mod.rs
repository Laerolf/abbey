/// Represents a Game blueprint.
pub struct GameBlueprint {
    /// The ID of the Player of the Game to create.
    pub player_id: i32,

    /// The ID of the Monastery of the Game to create.
    pub monastery_id: i32,

    /// The ID of the Surroundings of the Monastery in the Game to create.
    pub surroundings_id: i32,
}

impl GameBlueprint {
    /// Creates a new [`GameBlueprint`].
    pub fn new(player_id: i32, monastery_id: i32, surroundings_id: i32) -> Self {
        Self {
            player_id,
            monastery_id,
            surroundings_id,
        }
    }
}

/// Represents a form to assign a User to a Game.
pub struct UserGameAssignmentForm {
    /// The ID of the User.
    pub user_id: i32,
    /// The ID of the Game.
    pub game_id: i32,
}

impl UserGameAssignmentForm {
    /// Creates a new [`UserGameAssignmentForm`].
    pub fn new(user_id: i32, game_id: i32) -> Self {
        Self { user_id, game_id }
    }
}

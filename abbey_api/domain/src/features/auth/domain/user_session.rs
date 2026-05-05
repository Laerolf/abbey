use tracing::error;

use crate::{
    features::{
        auth::error::{AuthenticationErrorKind, GetUserSessionErrorKind},
        game::domain::Game,
        user::domain::User,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a session of a [User].
pub struct UserSession {
    /// The User of the session.
    user: User,
    /// The Game of the session.
    game: Option<Game>,
}

impl UserSession {
    /// Recreates a [`UserSession`] based on the provided parameters.
    pub fn from(user: User, game: Option<Game>) -> Self {
        Self { user, game }
    }

    /// Gets the [Game] of this [`UserSession`].
    pub fn game(&self) -> &Option<Game> {
        &self.game
    }

    /// Gets the [Game ID][Game::id] of the [`UserSession`].
    pub fn get_game_id(&self) -> Result<i32, DomainError<AuthenticationErrorKind>> {
        self.game
            .as_ref()
            .map(|game| game.id().unwrap())
            .ok_or_else(|| {
                DomainError::from(AuthenticationErrorKind::GetUserSession(
                    GetUserSessionErrorKind::GameNotFound,
                ))
            })
            .inspect_err(|_error| error!("Failed to find the ID of a user session game."))
    }
}

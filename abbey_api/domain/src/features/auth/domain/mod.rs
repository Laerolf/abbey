use crate::features::auth::domain::{refresh_token::RefreshToken, session_token::SessionToken};

pub mod refresh_token;
pub mod session_token;
pub mod user_session;

/// Authentication tokens used throughout the application.
pub enum AuthenticationTokens {
    /// The session token.
    SessionToken,
    /// The refresh token.
    RefreshToken,
}

impl AuthenticationTokens {
    /// Returns the header name of the [`AuthenticationToken`][AuthenticationTokens].
    pub fn header_name(&self) -> String {
        match &self {
            AuthenticationTokens::SessionToken => "Authorization".to_string(),
            AuthenticationTokens::RefreshToken => "X-refresh-token".to_string(),
        }
    }

    /// Returns the cookie name of the [`AuthenticationToken`][AuthenticationTokens].
    pub fn cookie_name(&self) -> String {
        match &self {
            AuthenticationTokens::SessionToken => "session_token".to_string(),
            AuthenticationTokens::RefreshToken => "refresh_token".to_string(),
        }
    }
}

pub struct AuthTokens {
    session_token: SessionToken,
    refresh_token: RefreshToken,
}

impl AuthTokens {
    /// Creates a [`AuthTokens`] element based on a [SessionToken] and a [RefreshToken].
    pub fn from(session_token: SessionToken, refresh_token: RefreshToken) -> Self {
        Self {
            session_token,
            refresh_token,
        }
    }

    /// Gets the [SessionToken] of this set of [`AuthTokens`].
    pub fn session_token(&self) -> &SessionToken {
        &self.session_token
    }

    /// Gets the [RefreshToken] of this set of [`AuthTokens`].
    pub fn refresh_token(&self) -> &RefreshToken {
        &self.refresh_token
    }
}

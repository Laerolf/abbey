use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{Error, PasswordHasher, SaltString},
};
use futures::TryFutureExt;
use rand::rngs::OsRng;
use sea_orm::{ConnectionTrait, DatabaseTransaction};
use time::{Duration, OffsetDateTime};
use tracing::{error, info};

use crate::{
    features::{
        auth::{
            domain::{
                AuthTokens, refresh_token::RefreshTokenValue, session_token::SessionToken,
                user_session::UserSession,
            },
            error::{
                AuthenticationErrorKind, GetUserSessionErrorKind, LoginErrorKind, RefreshErrorKind,
                RegistrationErrorKind,
            },
            forms::{LoginForm, RefreshTokenCreationForm, RegistrationForm},
            repository::RefreshTokenRepository,
        },
        game::domain::Game,
        user::{
            domain::User,
            error::{UserCreationErrorKind, UserErrorKind},
            forms::UserCreationForm,
            service::UserService,
        },
    },
    shared::error::DomainError,
};

const SESSION_TOKEN_LIFESPAN: Duration = Duration::minutes(15);
const REFRESH_TOKEN_LIFESPAN: Duration = Duration::weeks(2);

/// Represents a service handling the authentication topic.
#[derive(Clone)]
pub struct AuthenticationService {
    refresh_token_repository: RefreshTokenRepository,
    user_service: UserService,
}

impl AuthenticationService {
    /// Creates a new [`AuthenticationService`].
    pub fn new(
        refresh_token_repository: RefreshTokenRepository,
        user_service: UserService,
    ) -> Self {
        Self {
            refresh_token_repository,
            user_service,
        }
    }

    /// Hashes a password
    fn hash_password(&self, password: &str) -> Result<String, Error> {
        // TODO: Handle salt
        let salt = SaltString::generate(&mut OsRng);

        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string())
    }

    /// Validates whether a password matches a hashed password.
    fn validate_password(&self, expected: &str, actual: &str) -> Result<bool, Error> {
        let expected_parsed = PasswordHash::new(expected)?;
        Ok(Argon2::default()
            .verify_password(actual.as_bytes(), &expected_parsed)
            .is_ok())
    }

    /// Registers a new [User].
    pub async fn register(
        &self,
        form: RegistrationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<User, DomainError<AuthenticationErrorKind>> {
        let hashed_password = self
            .hash_password(&form.password)
            .inspect_err(|error| error!("{}", error))
            .map_err(|_error| {
                DomainError::from(AuthenticationErrorKind::Registration(
                    RegistrationErrorKind::HashPassword,
                ))
            })?;

        let user_creation_form = UserCreationForm::new(form.email.clone(), hashed_password);

        self.user_service
            .create_user(user_creation_form, db_transaction)
            .await
            .map_err(|error| match error.kind() {
                UserErrorKind::Creation(UserCreationErrorKind::EmailAlreadyExists(_)) => {
                    DomainError::from(AuthenticationErrorKind::Registration(
                        RegistrationErrorKind::EmailAlreadyExists,
                    ))
                    .with_context("email", form.email.clone())
                }
                _ => DomainError::from(AuthenticationErrorKind::Registration(
                    RegistrationErrorKind::CreateUser,
                ))
                .with_cause(error),
            })
    }

    /// Logins a User.
    pub async fn login(
        &self,
        form: LoginForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<AuthTokens, DomainError<AuthenticationErrorKind>> {
        let Some(user) = self
            .user_service
            .find_by_email(&form.email, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Login(
                    LoginErrorKind::UserDoesNotExist,
                ))
                .with_cause(error)
                .with_context("email", form.email.clone())
            })?
        else {
            return Err(DomainError::from(AuthenticationErrorKind::Login(
                LoginErrorKind::UserDoesNotExist,
            ))
            .with_context("email", form.email.clone()));
        };

        if !self
            .validate_password(&user.password().to_string(), &form.password)
            .inspect_err(|error| error!("{}", error))
            .map_err(|_error| {
                DomainError::from(AuthenticationErrorKind::Login(
                    LoginErrorKind::UserDoesNotExist,
                ))
            })?
        {
            return Err(DomainError::from(AuthenticationErrorKind::Login(
                LoginErrorKind::WrongPassword,
            )));
        }

        let game_id = user.games().first().map(|game| game.id().unwrap());

        let session_token_created_at = OffsetDateTime::now_utc();
        let session_token_expires_at =
            session_token_created_at.saturating_add(SESSION_TOKEN_LIFESPAN);

        let session_token = SessionToken::new(
            user.id().unwrap(),
            game_id,
            session_token_created_at,
            session_token_expires_at,
        );

        let refresh_token_created_at = OffsetDateTime::now_utc();
        let refresh_token_expires_at =
            refresh_token_created_at.saturating_add(REFRESH_TOKEN_LIFESPAN);
        let refresh_token_creation_form = RefreshTokenCreationForm::new(
            user.id().unwrap(),
            refresh_token_created_at,
            refresh_token_expires_at,
        );

        let refresh_token = self
            .refresh_token_repository
            .create(refresh_token_creation_form, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Login(
                    LoginErrorKind::CreateRefreshToken,
                ))
                .with_cause(error)
            })?;

        info!(
            "Successfully authenticated user with ID '{}'.",
            user.id().unwrap()
        );

        Ok(AuthTokens::from(session_token, refresh_token))
    }

    /// Refreshes a User's authentication tokens.
    pub async fn refresh(
        &self,
        refresh_token_value: RefreshTokenValue,
        db_transaction: &DatabaseTransaction,
    ) -> Result<AuthTokens, DomainError<AuthenticationErrorKind>> {
        let refresh_token = self
            .refresh_token_repository
            .get_by_value(refresh_token_value.to_string(), db_transaction)
            .await?;

        refresh_token
            .clone()
            .validate()
            .inspect_err(|error| error!("A refresh token is invalid: '{}'", error))?;

        self.refresh_token_repository
            .delete_by_id(&refresh_token.id().unwrap(), db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::DeleteRefreshToken,
                ))
                .with_cause(error)
                .with_context("id", refresh_token.id().unwrap().to_string())
            })?;

        let refresh_token_user = self
            .user_service
            .get_by_id_with_relations(refresh_token.user_id(), db_transaction)
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::CreateRefreshToken,
                ))
                .with_cause(error)
            })
            .await?;

        let game_id = refresh_token_user
            .games()
            .first()
            .map(|game| game.id().unwrap());

        let new_session_token_created_at = OffsetDateTime::now_utc();
        let new_session_token_expires_at =
            new_session_token_created_at.saturating_add(SESSION_TOKEN_LIFESPAN);

        let new_session_token = SessionToken::new(
            refresh_token_user.id().unwrap(),
            game_id,
            new_session_token_created_at,
            new_session_token_expires_at,
        );

        let new_refresh_token_created_at = OffsetDateTime::now_utc();
        let new_refresh_token_expires_at =
            new_refresh_token_created_at.saturating_add(REFRESH_TOKEN_LIFESPAN);

        let new_refresh_token_creation_form = RefreshTokenCreationForm::new(
            refresh_token_user.id().unwrap(),
            new_refresh_token_created_at,
            new_refresh_token_expires_at,
        );

        let new_refresh_token = self
            .refresh_token_repository
            .create(new_refresh_token_creation_form, db_transaction)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::CreateRefreshToken,
                ))
                .with_cause(error)
            })?;

        info!(
            "Successfully refreshed the authententication tokens for a user with ID '{}'.",
            refresh_token.user_id()
        );

        Ok(AuthTokens::from(new_session_token, new_refresh_token))
    }

    /// Gets the [`UserSession`] matching the provided [SessionToken].
    pub async fn get_user_session<C: ConnectionTrait>(
        &self,
        session_token: &SessionToken,
        db_connection: &C,
    ) -> Result<UserSession, DomainError<AuthenticationErrorKind>> {
        let user = self
            .user_service
            .find_by_id_with_relations(session_token.user_id(), db_connection)
            .await
            .inspect_err(|error| error!("Failed to find the user of a user session => {}", error))
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::GetUserSession(
                    GetUserSessionErrorKind::UserNotFound,
                ))
                .with_cause(error)
            })?
            .ok_or(DomainError::from(AuthenticationErrorKind::GetUserSession(
                GetUserSessionErrorKind::UserNotFound,
            )))?;

        let game: Option<Game> = user.games().iter().find_map(|user_game| {
            (user_game.id().unwrap() == session_token.game_id().unwrap()).then(|| user_game.clone())
        });

        Ok(UserSession::from(user, game))
    }
}

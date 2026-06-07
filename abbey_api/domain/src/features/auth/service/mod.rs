use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{Error, PasswordHasher, SaltString},
};
use futures::TryFutureExt;
use rand::rngs::OsRng;
use sea_orm::{ConnectionTrait, DatabaseTransaction};
use tracing::{error, info};

use crate::{
    features::{
        auth::{
            domain::{
                AuthTokens,
                refresh_token::{RefreshToken, RefreshTokenValue},
                session_token::SessionToken,
                user_session::UserSession,
            },
            error::{
                AuthenticationErrorKind, GetUserSessionErrorKind, LoginErrorKind, RefreshErrorKind,
                RegistrationErrorKind,
            },
            forms::{RefreshTokenBlueprint, UserLoginForm, UserRegistrationForm},
            mapper::RefreshTokenMapper,
            repository::RefreshTokenRepository,
        },
        game::domain::Game,
        user::{
            domain::User,
            error::{UserCreationErrorKind, UserErrorKind},
            forms::UserBlueprint,
            service::{UserCommandService, UserQueryService},
        },
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service handling the authentication topic.
#[derive(Clone)]
pub struct AuthenticationService {
    refresh_token_command_service: RefreshTokenCommandService,
    refresh_token_query_service: RefreshTokenQueryService,
    user_command_service: UserCommandService,
    user_query_service: UserQueryService,
}

impl AuthenticationService {
    /// Creates a new [`AuthenticationService`].
    pub fn new(
        refresh_token_command_service: RefreshTokenCommandService,
        refresh_token_query_service: RefreshTokenQueryService,
        user_command_service: UserCommandService,
        user_query_service: UserQueryService,
    ) -> Self {
        Self {
            refresh_token_command_service,
            refresh_token_query_service,
            user_command_service,
            user_query_service,
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
        form: UserRegistrationForm,
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

        let user_blueprint = UserBlueprint::new(form.email, hashed_password);

        self.user_command_service
            .create(user_blueprint.clone(), db_transaction)
            .await
            .map_err(|error| match error.kind() {
                UserErrorKind::Creation(UserCreationErrorKind::EmailAlreadyExists(_)) => {
                    DomainError::from(AuthenticationErrorKind::Registration(
                        RegistrationErrorKind::EmailAlreadyExists,
                    ))
                    .with_context("email", user_blueprint.email.clone())
                }
                _ => DomainError::from(AuthenticationErrorKind::Registration(
                    RegistrationErrorKind::CreateUser,
                ))
                .with_cause(error),
            })
    }

    /// Logins a User.
    pub async fn login<C: ConnectionTrait>(
        &self,
        form: UserLoginForm,
        db_connection: &C,
    ) -> Result<AuthTokens, DomainError<AuthenticationErrorKind>> {
        let user = self
            .user_query_service
            .get_by_email(&form.email, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Login(
                    LoginErrorKind::WrongCredentials,
                ))
                .with_cause(error)
            })?;

        if !self
            .validate_password(&user.password().to_string(), &form.password)
            .inspect_err(|error| error!("{}", error))
            .map_err(|_error| {
                DomainError::from(AuthenticationErrorKind::Login(
                    LoginErrorKind::WrongCredentials,
                ))
            })?
        {
            return Err(DomainError::from(AuthenticationErrorKind::Login(
                LoginErrorKind::WrongCredentials,
            )));
        }

        let game_id: Option<i32> = user.games().first().and_then(|game| game.id().ok());

        let user_id: i32 = user.id().map_err(|error| {
            DomainError::from(AuthenticationErrorKind::GetUserSession(
                GetUserSessionErrorKind::UserNotFound,
            ))
            .with_cause(error)
        })?;

        let session_token = SessionToken::new(&user_id, &game_id);

        let refresh_token = self
            .refresh_token_command_service
            .create(RefreshTokenBlueprint::new(&user_id), db_connection)
            .await?;

        info!(
            "Successfully authenticated user with ID '{}'.",
            user.id().unwrap()
        );

        Ok(AuthTokens::from(session_token, refresh_token))
    }

    /// Refreshes a User's authentication tokens.
    pub async fn refresh<C: ConnectionTrait>(
        &self,
        refresh_token_value: RefreshTokenValue,
        db_connection: &C,
    ) -> Result<AuthTokens, DomainError<AuthenticationErrorKind>> {
        let refresh_token = self
            .refresh_token_query_service
            .get_by_value(refresh_token_value.to_string(), db_connection)
            .await
            .map_err(|_error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::RefreshTokenNotFound,
                ))
            })?;

        refresh_token
            .clone()
            .validate()
            .inspect_err(|error| error!("A refresh token is invalid: '{}'", error))?;

        self.refresh_token_command_service
            .delete_by_id(&refresh_token.id()?, db_connection)
            .await?;

        let refresh_token_user = self
            .user_query_service
            .get_by_id(refresh_token.user_id(), db_connection)
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::CreateRefreshToken,
                ))
                .with_cause(error)
            })
            .await?;

        let user_id: i32 = refresh_token_user.id().map_err(|error| {
            DomainError::from(AuthenticationErrorKind::GetUserSession(
                GetUserSessionErrorKind::UserNotFound,
            ))
            .with_cause(error)
        })?;

        let game_id: Option<i32> = refresh_token_user
            .games()
            .first()
            .and_then(|game| game.id().ok());

        let new_session_token = SessionToken::new(&user_id, &game_id);

        let new_refresh_token = self
            .refresh_token_command_service
            .create(RefreshTokenBlueprint::new(&user_id), db_connection)
            .await?;

        info!(
            "Successfully refreshed the authententication tokens for a user with ID '{}'.",
            refresh_token.user_id()
        );

        Ok(AuthTokens::from(new_session_token, new_refresh_token))
    }
}

/// Represents a command service for [`RefreshTokens`][`RefreshToken`].
#[derive(Clone)]
pub struct RefreshTokenCommandService {
    repository: RefreshTokenRepository,
    refresh_token_query_service: RefreshTokenQueryService,
}

impl RefreshTokenCommandService {
    /// Creates a new [`RefreshTokenCommandService`].
    pub fn new(
        repository: RefreshTokenRepository,
        refresh_token_query_service: RefreshTokenQueryService,
    ) -> Self {
        Self {
            repository,
            refresh_token_query_service,
        }
    }

    /// Creates a new [`RefreshToken`].
    pub async fn create<C: ConnectionTrait>(
        &self,
        blueprint: RefreshTokenBlueprint,
        db_connection: &C,
    ) -> Result<RefreshToken, DomainError<AuthenticationErrorKind>> {
        if let Some(existing_model) = self
            .refresh_token_query_service
            .find_by_user_id(&blueprint.user_id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(RefreshErrorKind::FindById))
                    .with_cause(error)
            })?
        {
            self.delete(&existing_model, db_connection)
                .await
                .map_err(|error| {
                    DomainError::from(AuthenticationErrorKind::Refresh(
                        RefreshErrorKind::CreateRefreshToken,
                    ))
                    .with_cause(error)
                })?;
        }

        let new_model = self
            .repository
            .create(
                RefreshTokenMapper::to_new_active_model(blueprint),
                db_connection,
            )
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::CreateRefreshToken,
                ))
                .with_cause(error)
            })?;

        self.refresh_token_query_service
            .get_by_id(&new_model.id, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::CreateRefreshToken,
                ))
                .with_cause(error)
            })
    }

    /// Deletes a [`RefreshToken`].
    pub async fn delete<C: ConnectionTrait>(
        &self,
        refresh_token: &RefreshToken,
        db_connection: &C,
    ) -> Result<(), DomainError<AuthenticationErrorKind>> {
        self.repository
            .delete(
                RefreshTokenMapper::to_active_model(refresh_token)?,
                db_connection,
            )
            .await
    }

    /// Deletes a [`RefreshToken`] for the provided ID.
    pub async fn delete_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<(), DomainError<AuthenticationErrorKind>> {
        let model = self
            .refresh_token_query_service
            .get_by_id(id, db_connection)
            .await?;

        self.delete(&model, db_connection).await
    }
}

/// Represents a query service for [`RefreshTokens`][`RefreshToken`].
#[derive(Clone)]
pub struct RefreshTokenQueryService {
    repository: RefreshTokenRepository,
}

impl RefreshTokenQueryService {
    /// Creates a new [`RefreshTokenQueryService`].
    pub fn new(repository: RefreshTokenRepository) -> Self {
        Self { repository }
    }

    /// Finds a [`RefreshToken`] for the provided User ID.
    pub async fn find_by_user_id<C: ConnectionTrait>(
        &self,
        user_id: &i32,
        db_connection: &C,
    ) -> Result<Option<RefreshToken>, DomainError<AuthenticationErrorKind>> {
        Ok(self
            .repository
            .find_by_user_id(user_id, db_connection)
            .await?
            .map(RefreshTokenMapper::to_domain_entity))
    }

    /// Gets a [`RefreshToken`] for the provided ID.
    pub async fn get_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<RefreshToken, DomainError<AuthenticationErrorKind>> {
        let model = self.repository.get_by_id(id, db_connection).await?;

        Ok(RefreshTokenMapper::to_domain_entity(model))
    }

    /// Gets a [`RefreshToken`] for the provided User ID.
    pub async fn get_by_user_id<C: ConnectionTrait>(
        &self,
        user_id: &i32,
        db_connection: &C,
    ) -> Result<RefreshToken, DomainError<AuthenticationErrorKind>> {
        self.find_by_user_id(user_id, db_connection)
            .await?
            .ok_or(DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::GetByUserId,
            )))
    }

    /// Gets a [`RefreshToken`] for the provided RefreshToken value.
    pub async fn get_by_value<C: ConnectionTrait>(
        &self,
        value: impl Into<&String>,
        db_connection: &C,
    ) -> Result<RefreshToken, DomainError<AuthenticationErrorKind>> {
        self.repository
            .find_by_value(value, db_connection)
            .await?
            .ok_or(DomainError::from(AuthenticationErrorKind::Refresh(
                RefreshErrorKind::GetByValue,
            )))
            .map(RefreshTokenMapper::to_domain_entity)
    }
}

/// Represents a query service for [`UserSessions`][UserSession].
#[derive(Clone)]
pub struct UserSessionQueryService {
    user_query_service: UserQueryService,
}

impl UserSessionQueryService {
    /// Creates a new [`UserSessionQueryService`].
    pub fn new(user_query_service: UserQueryService) -> Self {
        Self { user_query_service }
    }

    /// Gets a [`UserSession`] by a [SessionToken].
    pub async fn get_by_session_token<C: ConnectionTrait>(
        &self,
        session_token: &SessionToken,
        db_connection: &C,
    ) -> Result<UserSession, DomainError<AuthenticationErrorKind>> {
        let user = self
            .user_query_service
            .get_by_id(session_token.user_id(), db_connection)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::GetUserSession(
                    GetUserSessionErrorKind::UserNotFound,
                ))
                .with_cause(error)
            })?;

        let game: Option<Game> = user.games().iter().find_map(|user_game| {
            (user_game.id().unwrap() == session_token.game_id().unwrap()).then(|| user_game.clone())
        });

        Ok(UserSession::from(user, game))
    }
}

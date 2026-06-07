use entity::refresh_tokens;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::{
    features::{
        auth::{
            domain::refresh_token::RefreshToken,
            dto::{LoginUserRequest, RegisterUserRequest, RegisterUserResponse},
            error::{
                AuthenticationErrorKind, LoginErrorKind,
                RefreshErrorKind::{self},
                RegistrationErrorKind,
            },
            forms::{RefreshTokenBlueprint, UserLoginForm, UserRegistrationForm},
        },
        user::domain::User,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that maps authentication elements.
pub struct AuthenticationDtoMapper;

impl AuthenticationDtoMapper {
    /// Maps a [RegisterUserRequest] to a [`RegistrationForm`].
    pub fn to_registration_form(
        request: RegisterUserRequest,
    ) -> Result<UserRegistrationForm, DomainError<AuthenticationErrorKind>> {
        let Some(email_address) = request.email else {
            return Err(DomainError::from(AuthenticationErrorKind::Registration(
                RegistrationErrorKind::EmailRequired,
            )));
        };

        let Some(password) = request.password else {
            return Err(DomainError::from(AuthenticationErrorKind::Registration(
                RegistrationErrorKind::PasswordRequired,
            )));
        };

        let Some(confirmed_password) = request.confirmed_password else {
            return Err(DomainError::from(AuthenticationErrorKind::Registration(
                RegistrationErrorKind::ConfirmedPasswordRequired,
            )));
        };

        if password != confirmed_password {
            return Err(DomainError::from(AuthenticationErrorKind::Registration(
                RegistrationErrorKind::InvalidConfirmedPassword,
            )));
        }

        Ok(UserRegistrationForm::new(email_address, password))
    }

    /// Maps a [LoginUserRequest] to a [`LoginForm`].
    pub fn to_login_form(
        request: LoginUserRequest,
    ) -> Result<UserLoginForm, DomainError<AuthenticationErrorKind>> {
        let Some(email_address) = request.email else {
            return Err(DomainError::from(AuthenticationErrorKind::Login(
                LoginErrorKind::EmailRequired,
            )));
        };

        let Some(password) = request.password else {
            return Err(DomainError::from(AuthenticationErrorKind::Login(
                LoginErrorKind::PasswordRequired,
            )));
        };

        Ok(UserLoginForm::new(email_address, password))
    }

    /// Maps a [`User`] to a [RegisterUserResponse].
    pub fn to_response(entity: User) -> RegisterUserResponse {
        RegisterUserResponse {
            id: entity.id().unwrap(),
            email: entity.email().to_string(),
        }
    }
}

/// Represents an element that maps [`RefreshTokens`][RefreshToken].
pub struct RefreshTokenMapper;

impl RefreshTokenMapper {
    /// Creates a new [RefreshToken active model][`refresh_tokens::ActiveModel`].
    pub fn to_new_active_model(blueprint: RefreshTokenBlueprint) -> refresh_tokens::ActiveModel {
        refresh_tokens::ActiveModel {
            id: NotSet,
            created_at: Set(blueprint.created_at),
            last_updated_at: NotSet,
            user_id: Set(blueprint.user_id),
            value: Set(blueprint.value),
            expires_at: Set(blueprint.expires_at),
        }
    }

    /// Creates a [RefreshToken][refresh_tokens::ActiveModel].
    pub fn to_active_model(
        refresh_token: &RefreshToken,
    ) -> Result<refresh_tokens::ActiveModel, DomainError<AuthenticationErrorKind>> {
        Ok(refresh_tokens::ActiveModel {
            id: Set(refresh_token.id()?),
            created_at: Set(refresh_token.created_at().ok_or_else(|| {
                DomainError::from(AuthenticationErrorKind::Refresh(
                    RefreshErrorKind::NotPersistedYet,
                ))
            })?),
            last_updated_at: Set(*refresh_token.last_updated_at()),
            user_id: Set(*refresh_token.user_id()),
            value: Set(refresh_token.value().to_string().clone()),
            expires_at: Set(*refresh_token.expires_at()),
        })
    }

    /// Maps a [model][refresh_tokens::Model] to a [`RefreshToken`].
    pub fn to_domain_entity(model: refresh_tokens::Model) -> RefreshToken {
        RefreshToken::from(
            model.id,
            model.created_at,
            model.last_updated_at,
            model.value,
            model.user_id,
            model.expires_at,
        )
    }
}

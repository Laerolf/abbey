use entity::refresh_tokens;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::{
    features::{
        auth::{
            domain::refresh_token::RefreshToken,
            dto::{LoginUserRequest, RegisterUserRequest, RegisterUserResponse},
            error::{AuthenticationErrorKind, LoginErrorKind, RegistrationErrorKind},
            forms::{LoginForm, RefreshTokenCreationForm, RegistrationForm},
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
    ) -> Result<RegistrationForm, DomainError<AuthenticationErrorKind>> {
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

        Ok(RegistrationForm::new(email_address, password))
    }

    /// Maps a [LoginUserRequest] to a [`LoginForm`].
    pub fn to_login_form(
        request: LoginUserRequest,
    ) -> Result<LoginForm, DomainError<AuthenticationErrorKind>> {
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

        Ok(LoginForm::new(email_address, password))
    }

    /// Maps a [`User`] to a [RegisterUserResponse].
    pub fn to_response(entity: User) -> RegisterUserResponse {
        RegisterUserResponse {
            id: entity.id().unwrap(),
            email: entity.email().to_string(),
        }
    }
}

/// Represents an element that maps [RefreshToken][`super::domain::RefreshToken`]s.
pub struct RefreshTokenMapper;

impl RefreshTokenMapper {
    /// Creates a new [RefreshToken active model][`refresh_tokens::ActiveModel`].
    pub fn to_new_active_model(
        creation_form: RefreshTokenCreationForm,
    ) -> refresh_tokens::ActiveModel {
        refresh_tokens::ActiveModel {
            id: NotSet,
            created_at: Set(creation_form.created_at),
            last_updated_at: NotSet,
            user_id: Set(creation_form.user_id),
            value: Set(creation_form.value),
            expires_at: Set(creation_form.expires_at),
        }
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

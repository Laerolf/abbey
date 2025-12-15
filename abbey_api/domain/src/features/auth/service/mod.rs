use crate::{
    features::{
        auth::{
            error::{AuthenticationErrorKind, RegistrationErrorKind},
            forms::RegistrationForm,
        },
        user::{forms::UserCreationForm, repository::UserWithRelations, service::UserService},
    },
    shared::error::DomainError,
};

/// Represents a service handling the authentication topic.
#[derive(Clone)]
pub struct AuthenticationService {
    user_service: UserService,
}

impl AuthenticationService {
    /// Creates a new [`AuthenticationService`].
    pub fn new(user_service: UserService) -> Self {
        AuthenticationService { user_service }
    }

    /// Registers a new user.
    pub async fn register(
        &self,
        form: RegistrationForm,
    ) -> Result<UserWithRelations, DomainError<AuthenticationErrorKind>> {
        let email = form
            .email
            .ok_or(DomainError::from(AuthenticationErrorKind::Registration(
                RegistrationErrorKind::EmailRequired,
            )))?;

        let user_creation_form = UserCreationForm::new(email);

        let user = self
            .user_service
            .create_user(user_creation_form)
            .await
            .map_err(|error| {
                DomainError::from(AuthenticationErrorKind::Registration(
                    RegistrationErrorKind::CreateUser,
                ))
                .with_cause(error)
            })?;

        Ok(user)
    }
}

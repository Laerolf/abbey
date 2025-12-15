use std::fmt::Display;

use crate::shared::error::DomainErrorKind;

#[derive(Debug)]
pub enum RegistrationErrorKind {
    EmailRequired,
    EmailInvalid(String),
    EmailAlreadyExists(String),
    CreateUser,
}

#[derive(Debug)]
pub enum AuthenticationErrorKind {
    /// Failed to register a new user.
    Registration(RegistrationErrorKind),
}

impl DomainErrorKind for AuthenticationErrorKind {
    /// Gets the locale code of this [AuthenticationError].
    fn code(&self) -> String {
        match self {
            Self::Registration(err) => match err {
                RegistrationErrorKind::EmailRequired => {
                    "error.authentication.registration.email_required".to_string()
                }
                RegistrationErrorKind::EmailInvalid(_) => {
                    "error.authentication.registration.email_invalid".to_string()
                }
                RegistrationErrorKind::EmailAlreadyExists(_) => {
                    "error.authentication.registration.email_exists".to_string()
                }
                RegistrationErrorKind::CreateUser => {
                    "error.authentication.registration.create_user".to_string()
                }
            },
        }
    }

    /// Gets the message of this [UserError].
    fn message(&self) -> String {
        match self {
            Self::Registration(err) => match err {
                RegistrationErrorKind::EmailRequired => "Email is required.".to_string(),
                RegistrationErrorKind::EmailInvalid(email) => {
                    format!("Email '{}' is invalid.", email)
                }
                RegistrationErrorKind::EmailAlreadyExists(email) => {
                    format!("Email '{}' already exists.", email)
                }
                RegistrationErrorKind::CreateUser => "Failed to create a user.".to_string(),
            },
        }
    }
}

impl Display for AuthenticationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

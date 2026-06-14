use axum::{Json, Router, extract::State, routing::post};
use domain::{
    features::{
        auth::{
            domain::refresh_token::RefreshTokenValue,
            dto::{
                LoginUserRequest, LoginUserResponse, RefreshUserResponse, RegisterUserRequest,
                RegisterUserResponse,
            },
            mapper::AuthenticationDtoMapper,
        },
        game::dto::GameOptionsForm,
    },
    shared::DomainElement,
};
use sea_orm::DatabaseConnection;
use tracing::error;
use utoipa::OpenApi;

use crate::{
    error::AppError,
    shared::{ApiContext, ApiFeature},
};

pub struct Feature;

impl ApiFeature for Feature {
    fn routes() -> Router<ApiContext<DatabaseConnection>> {
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .route("/refresh", post(refresh))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(register, login, refresh),
    tags((name = "Authentication"))
)]
pub struct AuthApiDoc;

/// Registers a new User.
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/auth/register",
    responses(
        (status = 200, description = "The user was registered.", body = RegisterUserResponse),
        (status = 500, description = "The user was not registered due to an error.", body = AppError)
    ),
    tag = "Authentication"
)]
async fn register(
    State(context): State<ApiContext<DatabaseConnection>>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, AppError> {
    let registration_form = AuthenticationDtoMapper::to_registration_form(payload.clone())?;
    let game_options = payload
        .game_options
        .map_or(GameOptionsForm::empty(), |create_game_request| {
            GameOptionsForm::from(create_game_request)
        });

    let new_user = context
        .in_transaction(async |db_transaction| {
            context
                .authentication_service
                .register(registration_form, game_options, db_transaction)
                .await
                .inspect_err(|error| {
                    error!(?error);
                })
        })
        .await?;

    Ok(Json(RegisterUserResponse::new(
        new_user.id()?,
        new_user.email().to_string(),
    )))
}

/// Handles a login attempt of User.
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/auth/login",
    responses(
        (status = 200, description = "The login attempt was successful.", body = LoginUserResponse),
        (status = 401, description = "The login attempt failed due to wrong credentials.", body = AppError),
        (status = 500, description = "The login attempt failed due to an error.", body = AppError)
    ),
    tag = "Authentication"
)]
async fn login(
    State(context): State<ApiContext<DatabaseConnection>>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<LoginUserResponse, AppError> {
    let login_attempt_form = AuthenticationDtoMapper::to_login_form(payload)?;

    let auth_tokens = context
        .in_transaction(async |transaction| {
            context
                .authentication_service
                .login(login_attempt_form, transaction)
                .await
                .inspect_err(|error| {
                    error!(?error);
                })
        })
        .await?;

    let response = LoginUserResponse::new(
        auth_tokens.session_token().to_jwt()?,
        auth_tokens.refresh_token().clone(),
    );

    Ok(response)
}

/// Handles a refresh attempt of a User.
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    responses(
        (status = 200, description = "The refresh attempt was successful.", body = RefreshUserResponse),
        (status = 500, description = "The refresh attempt failed due to an error.", body = AppError)
    ),
    params(
        ("refresh_token" = String, Cookie, description = "A refresh token."),
    ),
    tag = "Authentication"
)]
async fn refresh(
    State(context): State<ApiContext<DatabaseConnection>>,
    refresh_token_value: RefreshTokenValue,
) -> Result<RefreshUserResponse, AppError> {
    let refreshed_auth_tokens = context
        .in_transaction(async |db_transaction| {
            context
                .authentication_service
                .refresh(refresh_token_value, db_transaction)
                .await
                .inspect_err(|error| {
                    error!(?error);
                })
        })
        .await?;

    let response = RefreshUserResponse::new(
        refreshed_auth_tokens.session_token().to_jwt()?,
        refreshed_auth_tokens.refresh_token().clone(),
    );

    Ok(response)
}

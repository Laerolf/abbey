use axum::{Json, Router, extract::State, routing::get};
use domain::{
    features::{
        auth::domain::session_token::SessionToken,
        game::{dto::GameDto, error::GameErrorKind},
    },
    shared::error::DomainError,
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
    fn routes() -> axum::Router<ApiContext<DatabaseConnection>> {
        Router::new().route("/me", get(get_session_game))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_session_game),
    tags((name = "Games"))
)]
pub struct GamesApiDoc;

/// Gets a User for the provided session.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/games/me",
    responses(
        (status = 200, description = "The game was found.", body = GameDto),
        (
            status = 401,
            description = "Failed to authenticate a user.",
            body = AppError,
            example = json!({
                "message": "Failed to authenticate a user.",
                "code": "error.authentication.authenticate",
            })
        ),
        (
            status = 404,
            description = "Failed to find a game for the provided ID.",
            body = AppError,
            example = json!({
                "message": "Failed to find a game by its ID.",
                "code": "error.game.get_by_id",
            })
        )
    ),
    security(("api_auth" = [])),
    tag = "Games"
)]
async fn get_session_game(
    State(context): State<ApiContext<DatabaseConnection>>,
    session_token: SessionToken,
) -> Result<Json<GameDto>, AppError> {
    let game = context
        .game_query_service
        .get_by_id(
            &session_token
                .game_id()
                .ok_or(DomainError::from(GameErrorKind::GetById))?,
            context.db_connection(),
        )
        .await
        .inspect_err(|error| {
            error!(?error);
        })?;

    Ok(Json(GameDto::from(game)))
}

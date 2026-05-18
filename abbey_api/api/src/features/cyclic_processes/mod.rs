use axum::{
    Json, Router,
    extract::{Path, State},
    routing::post,
};
use domain::{
    features::{
        assignment::{
            dto::{ProcessAssignmentDto, ProcessAssignmentRequest},
            error::AssignmentErrorKind,
            forms::ProcessAssignmentForm,
        },
        auth::domain::session_token::SessionToken,
        process::dto::ProcessDto,
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
    fn routes() -> Router<ApiContext<DatabaseConnection>> {
        Router::new()
            .route("/assign", post(assign))
            .route("/{process_id}/start", post(start))
            .route("/{process_id}/pause", post(pause))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(assign, start, pause),
    tags((name = "Cyclic Processes"))
)]
pub struct CyclicProcessesApiDoc;

/// Assigns a cyclic process to an actor.
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/cyclic-processes/assign",
    responses(
        (status = 200, description = "The cyclic process has been assigned to an actor.", body = ProcessAssignmentDto),
        (status = 401, description = "The cyclic process was not able to be assigned due to you being anonymous.", body = AppError),
        (status = 404, description = "The cyclic process was not found.", body = AppError),
        (status = 500, description = "The cyclic process was not able to be assigned due to an error.", body = AppError),
    ),
    tag = "Cyclic Processes"
)]
async fn assign(
    State(context): State<ApiContext<DatabaseConnection>>,
    session_token: SessionToken,
    Json(payload): Json<ProcessAssignmentRequest>,
) -> Result<Json<ProcessAssignmentDto>, AppError> {
    let game = context
        .authentication_service
        .get_user_session(&session_token, context.db_connection())
        .await?
        .game()
        .clone()
        .ok_or_else(|| DomainError::from(AssignmentErrorKind::Unknown))?;

    let assignment = context
        .in_transaction(async |db_transaction| {
            context
                .process_assignment_service
                .assign_process_to_actors_in_game(
                    ProcessAssignmentForm::from_request(payload),
                    &game,
                    db_transaction,
                )
                .await
                .inspect_err(|error| {
                    error!(?error);
                })
        })
        .await?;

    Ok(Json(ProcessAssignmentDto::from(assignment)))
}

/// Starts a cyclic process.
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/cyclic-processes/{process_id}/start",
    responses(
        (status = 200, description = "The cyclic process has started.", body = ProcessDto),
        (status = 401, description = "The cyclic process was not able to start due to you being anonymous.", body = AppError),
        (status = 404, description = "The cyclic process was not found.", body = AppError),
        (status = 500, description = "The cyclic process was not able to start due to an error.", body = AppError),
    ),
    tag = "Cyclic Processes"
)]
async fn start(
    State(context): State<ApiContext<DatabaseConnection>>,
    Path(process_id): Path<i32>,
    session_token: SessionToken,
) -> Result<Json<ProcessDto>, AppError> {
    let game_id = context
        .authentication_service
        .get_user_session(&session_token, context.db_connection())
        .await?
        .get_game_id()?;

    let cyclic_process = context
        .in_transaction(async |db_transaction| {
            context
                .cyclic_process_service
                .start_by_id_in_game(&process_id, &game_id, db_transaction)
                .await
                .inspect_err(|error| {
                    error!(?error);
                })
        })
        .await?;

    Ok(Json(ProcessDto::from(cyclic_process)))
}

/// Starts a cyclic process.
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/cyclic-processes/{process_id}/pause",
    responses(
        (status = 200, description = "The cyclic process has been paused.", body = ProcessDto),
        (status = 401, description = "The cyclic process was not able to pause due to you being anonymous.", body = AppError),
        (status = 404, description = "The cyclic process was not found.", body = AppError),
        (status = 500, description = "The cyclic process was not able to pause due to an error.", body = AppError),
    ),
    tag = "Cyclic Processes"
)]
async fn pause(
    State(context): State<ApiContext<DatabaseConnection>>,
    Path(process_id): Path<i32>,
    session_token: SessionToken,
) -> Result<Json<ProcessDto>, AppError> {
    let game_id = context
        .authentication_service
        .get_user_session(&session_token, context.db_connection())
        .await?
        .get_game_id()?;

    let cyclic_process = context
        .in_transaction(async |db_transaction| {
            context
                .cyclic_process_service
                .pause_by_id_in_game(&process_id, &game_id, db_transaction)
                .await
                .inspect_err(|error| {
                    error!(?error);
                })
        })
        .await?;

    Ok(Json(ProcessDto::from(cyclic_process)))
}

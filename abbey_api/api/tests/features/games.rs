use axum::http::StatusCode;
use domain::{
    features::{
        auth::forms::{LoginForm, RegistrationForm},
        game::dto::GameDto,
        monastery::dto::MonasteryDto,
        monk::{domain::Monk, dto::MonkDto},
        player::dto::PlayerDto,
        process::dto::{CyclicProcessDto, ProcessStatusDto},
        source::dto::SourceDto,
        surroundings::dto::SurroundingsDto,
    },
    shared::{DomainElement, DurationDto},
};

use serde_json::json;
use serial_test::serial;
use tower_cookies::cookie::time::Duration;

use crate::shared::{
    TestApp,
    fixtures::{TestUserFixture, test_user_fixture},
    utils::read_body_as_value,
};

#[tokio::test]
#[serial]
pub async fn test_get_session_game_returns_200() {
    // Given
    let TestUserFixture { email, password } = test_user_fixture();

    let app = TestApp::new().await;

    app.context
        .in_transaction(async |db_transaction| {
            app.context
                .authentication_service
                .register(RegistrationForm::new(&email, &password), db_transaction)
                .await
        })
        .await
        .expect("Failed to register the test user.");

    let test_auth_tokens = app
        .context
        .in_transaction(async |db_transaction| {
            app.context
                .authentication_service
                .login(LoginForm::new(email, password), db_transaction)
                .await
        })
        .await
        .expect("Failed to login the test user.");

    // When
    let response = app
        .get("/api/games/me")
        .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
        .send()
        .await;

    // Then
    assert_eq!(StatusCode::OK, response.status());

    let expected_dto = GameDto {
        id: 1,
        player: PlayerDto {
            id: 1,
            assigned_process_id: None,
        },
        monastery: MonasteryDto {
            id: 1,
            monks: (1..11)
                .map(|monk_id| MonkDto {
                    id: monk_id,
                    name: "Maurits".to_string(),
                    skill_ids: (1..3).collect(),
                    assigned_process_id: None,
                })
                .collect(),
        },
        surroundings: SurroundingsDto {
            id: 1,
            sources: (1..2)
                .map(|source_id| SourceDto {
                    id: source_id,
                    name: "the_beach".to_string(),
                    process: CyclicProcessDto {
                        id: 1,
                        cycle_interval: DurationDto::from(Duration::seconds(60)),
                        elapsed: DurationDto::from(Duration::milliseconds(0)),
                        paused_at: None,
                        started_at: None,
                        status: ProcessStatusDto::New,
                        assigned_actors: Vec::new(),
                    },
                    last_claim_at: None,
                })
                .collect(),
        },
    };

    let actual_dto: GameDto = read_body_as_value(response).await;

    assert_eq!(expected_dto, actual_dto)
}

#[tokio::test]
#[serial]
pub async fn test_get_session_game_returns_200_after_a_process_was_assigned_to_an_actor() {
    // Given
    let TestUserFixture { email, password } = test_user_fixture();

    let app = TestApp::new().await;
    let db_connection = app.context.db_connection();

    app.context
        .in_transaction(async |db_transaction| {
            app.context
                .authentication_service
                .register(RegistrationForm::new(&email, &password), db_transaction)
                .await
        })
        .await
        .expect("Failed to register the test user.");

    let test_auth_tokens = app
        .context
        .in_transaction(async |db_transaction| {
            app.context
                .authentication_service
                .login(LoginForm::new(email, password), db_transaction)
                .await
        })
        .await
        .expect("Failed to login the test user.");

    let game_id: i32 = app
        .context
        .user_session_query_service
        .get_by_session_token(test_auth_tokens.session_token(), db_connection)
        .await
        .expect("Failed to get the test user session.")
        .get_game_id()
        .expect("Failed to get the test game's ID");

    let game = app
        .context
        .game_query_service
        .get_by_id(&game_id, db_connection)
        .await
        .expect("Failed to get the test game.");

    let first_5_monks: Vec<Monk> = game.monastery().monks().clone().split_off(4);
    let first_5_monk_ids: Vec<i32> = first_5_monks
        .iter()
        .map(|monk| monk.id().unwrap())
        .collect();

    let first_source_cyclic_process = game
        .surroundings()
        .sources()
        .first()
        .expect("Failed to get the first source of the test surroundings.")
        .process()
        .clone();

    app.post("/api/cyclic-processes/assign")
        .body(&json!({
            "assign_player": false,
            "actor_ids": first_5_monk_ids,
            "process_id": first_source_cyclic_process.id().unwrap()
        }))
        .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
        .send()
        .await;

    // When
    let response = app
        .get("/api/games/me")
        .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
        .send()
        .await;

    // Then
    assert_eq!(StatusCode::OK, response.status());

    let response_game: GameDto = read_body_as_value(response).await;

    let first_source_cyclic_process_dto: CyclicProcessDto = response_game
        .surroundings
        .sources
        .first()
        .expect("Failed to get the DTO of the first source of the test surroundings.")
        .process
        .clone();

    let actual_assigned_monk_ids: Vec<i32> = first_source_cyclic_process_dto
        .assigned_actors
        .iter()
        .map(|actor_dto| actor_dto.id())
        .collect();

    assert_eq!(first_5_monk_ids, actual_assigned_monk_ids);
}

#[tokio::test]
#[serial]
pub async fn test_get_session_game_returns_401_when_not_authenticated() {
    // Given
    let app = TestApp::new().await;

    // When
    let response = app.get("/api/games/me").send().await;

    // Then
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

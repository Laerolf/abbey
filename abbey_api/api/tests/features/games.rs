use axum::http::StatusCode;
use domain::features::{
    actor::dto::MonkDto,
    auth::forms::{LoginForm, RegistrationForm},
    game::dto::GameDto,
    monastery::dto::MonasteryDto,
    player::dto::PlayerDto,
    process::dto::CyclicProcessDto,
    source::dto::SourceDto,
    surroundings::dto::SurroundingsDto,
};

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
            process: None,
        },
        monastery: MonasteryDto {
            id: 1,
            monks: (1..11)
                .map(|monk_id| MonkDto {
                    id: monk_id,
                    name: "Maurits".to_string(),
                    skill_ids: (1..3).collect(),
                    assigned_process: None,
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
                        cycle_interval: Duration::new(60, 0),
                        elapsed: Duration::new(0, 0),
                        paused_at: None,
                        started_at: None,
                        status: "new".to_string(),
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
pub async fn test_get_session_game_returns_401_when_not_authenticated() {
    // Given
    let app = TestApp::new().await;

    // When
    let response = app.get("/api/games/me").send().await;

    // Then
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

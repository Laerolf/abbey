pub mod start_cyclic_process {
    use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

    use axum::http::StatusCode;
    use domain::{
        features::{
            actor::{domain::ActorKind, dto::ActorDto},
            assignment::forms::ProcessAssignmentForm,
            auth::forms::{LoginForm, RegistrationForm},
            process::{
                dto::{CyclicProcessDto, ProcessStatusDto},
                error::ProcessErrorKind,
            },
        },
        shared::{
            DomainElement, DurationDto,
            error::{DomainError, DomainErrorKind},
        },
    };

    use serde_json::json;
    use serial_test::serial;

    use crate::shared::{
        TestApp,
        fixtures::{TestUserFixture, test_user_fixture},
        utils::{read_body_as_json, read_body_as_value},
    };

    #[tokio::test]
    #[serial]
    pub async fn test_start_cyclic_process_returns_200() {
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

        let user_session = app
            .context
            .user_session_query_service
            .get_by_session_token(test_auth_tokens.session_token(), db_connection)
            .await
            .expect("Failed to get the test user session.");

        let player = user_session
            .game()
            .as_ref()
            .expect("Failed to get the test game")
            .player();

        let game_id: i32 = user_session
            .get_game_id()
            .expect("Failed to get the test game's ID");

        let game = app
            .context
            .game_query_service
            .get_by_id(&game_id, db_connection)
            .await
            .expect("Failed to get the test game.");

        let first_source_cyclic_process = game
            .surroundings()
            .sources()
            .first()
            .expect("Failed to get the first source of the test surroundings.")
            .process()
            .clone();

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .process_command_service
                    .assign_process_to_actors_in_game(
                        ProcessAssignmentForm::new(
                            true,
                            Vec::new(),
                            first_source_cyclic_process.id().unwrap(),
                        ),
                        &game,
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to assign the test source cyclic process.");

        // When
        let response = app
            .post(
                &format!(
                    "/api/cyclic-processes/{}/start",
                    first_source_cyclic_process.id().unwrap()
                )
                .to_string(),
            )
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let expected_dto = CyclicProcessDto {
            id: 1,
            cycle_interval: DurationDto::from(Duration::seconds(60)),
            elapsed: DurationDto::from(Duration::milliseconds(0)),
            started_at: Some(OffsetDateTime::now_utc().format(&Rfc3339).unwrap()),
            paused_at: None,
            status: ProcessStatusDto::InProgress,
            assigned_actors: vec![ActorDto::from(ActorKind::Player(player.clone()))],
        };

        let actual_dto: CyclicProcessDto = read_body_as_value(response).await;

        assert_eq!(expected_dto.id, actual_dto.id);
        assert_eq!(expected_dto.cycle_interval, actual_dto.cycle_interval);
        assert_eq!(expected_dto.paused_at, actual_dto.paused_at);
        assert_eq!(expected_dto.status, actual_dto.status);
        assert_eq!(expected_dto.elapsed, actual_dto.elapsed);

        assert!(actual_dto.started_at.is_some());
        assert!(expected_dto.started_at.unwrap() > actual_dto.started_at.unwrap());
    }

    #[tokio::test]
    #[serial]
    pub async fn test_start_cyclic_process_returns_401_when_no_session_token_was_provided() {
        // Given

        let process_id = 1;

        let app = TestApp::new().await;

        // When
        let response = app
            .post(&format!("/api/cyclic-processes/{}/start", process_id).to_string())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    #[tokio::test]
    #[serial]
    pub async fn test_start_cyclic_process_returns_404_when_the_process_was_not_found() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();
        let not_existing_process_id = 666;

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
            .post(&format!("/api/cyclic-processes/{}/start", not_existing_process_id).to_string())
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::NOT_FOUND, response.status());

        let expected_error = DomainError::from(ProcessErrorKind::NotFound);

        let body = read_body_as_json(response).await;
        let expected_error_message = json!({
            "code": expected_error.kind().code(),
            "message": expected_error.kind().message()
        });

        assert_eq!(expected_error_message, body);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_start_cyclic_process_returns_500_when_the_process_has_no_assigned_actors() {
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

        let first_source_cyclic_process = game
            .surroundings()
            .sources()
            .first()
            .expect("Failed to get the first source of the test surroundings.")
            .process()
            .clone();

        // When
        let response = app
            .post(
                &format!(
                    "/api/cyclic-processes/{}/start",
                    first_source_cyclic_process.id().unwrap()
                )
                .to_string(),
            )
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());

        let expected_error = DomainError::from(ProcessErrorKind::Start)
            .with_cause(DomainError::from(ProcessErrorKind::NoAssignedPeople));

        let body = read_body_as_json(response).await;
        let expected_error_message = json!({
            "code": expected_error.kind().code(),
            "message": expected_error.kind().message(),
            "context": {
                "process_id": first_source_cyclic_process.id().unwrap().to_string()
            }
        });

        assert_eq!(expected_error_message, body);
    }
}

pub mod pause_cyclic_process {
    use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

    use axum::http::StatusCode;
    use domain::{
        features::{
            actor::{domain::ActorKind, dto::ActorDto},
            assignment::forms::ProcessAssignmentForm,
            auth::forms::{LoginForm, RegistrationForm},
            process::{
                dto::{CyclicProcessDto, ProcessStatusDto},
                error::ProcessErrorKind,
            },
        },
        shared::{
            DomainElement, DurationDto,
            error::{DomainError, DomainErrorKind},
        },
    };

    use serde_json::json;
    use serial_test::serial;

    use crate::shared::{
        TestApp,
        fixtures::{TestUserFixture, test_user_fixture},
        utils::{read_body_as_json, read_body_as_value},
    };

    #[tokio::test]
    #[serial]
    pub async fn test_pause_cyclic_process_returns_200() {
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

        let user_session = app
            .context
            .user_session_query_service
            .get_by_session_token(test_auth_tokens.session_token(), db_connection)
            .await
            .expect("Failed to get the test user session.");

        let player = user_session
            .game()
            .as_ref()
            .expect("Failed to get the test game")
            .player();

        let game_id: i32 = user_session
            .get_game_id()
            .expect("Failed to get the test game's ID");

        let game = app
            .context
            .game_query_service
            .get_by_id(&game_id, db_connection)
            .await
            .expect("Failed to get the test game.");

        let first_source_cyclic_process = game
            .surroundings()
            .sources()
            .first()
            .expect("Failed to get the first source of the test surroundings.")
            .process()
            .clone();

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .process_command_service
                    .assign_process_to_actors_in_game(
                        ProcessAssignmentForm::new(
                            true,
                            Vec::new(),
                            first_source_cyclic_process.id().unwrap(),
                        ),
                        &game,
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to assign the test source cyclic process.");

        app.post(
            &format!(
                "/api/cyclic-processes/{}/start",
                first_source_cyclic_process.id().unwrap()
            )
            .to_string(),
        )
        .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
        .send()
        .await;

        // When
        let response = app
            .post(
                &format!(
                    "/api/cyclic-processes/{}/pause",
                    first_source_cyclic_process.id().unwrap()
                )
                .to_string(),
            )
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let expected_dto = CyclicProcessDto {
            id: 1,
            cycle_interval: DurationDto::from(Duration::seconds(60)),
            elapsed: DurationDto::from(Duration::milliseconds(0)),
            started_at: Some(OffsetDateTime::now_utc().format(&Rfc3339).unwrap()),
            paused_at: Some(OffsetDateTime::now_utc().format(&Rfc3339).unwrap()),
            status: ProcessStatusDto::Paused,
            assigned_actors: vec![ActorDto::from(ActorKind::Player(player.clone()))],
        };

        let actual_dto: CyclicProcessDto = read_body_as_value(response).await;

        assert_eq!(expected_dto.id, actual_dto.id);
        assert_eq!(expected_dto.cycle_interval, actual_dto.cycle_interval);
        assert_eq!(expected_dto.status, actual_dto.status);
        assert_eq!(expected_dto.elapsed, actual_dto.elapsed);

        assert!(actual_dto.started_at.is_none());

        assert!(actual_dto.paused_at.is_some());
        assert!(expected_dto.paused_at.unwrap() > actual_dto.paused_at.unwrap());
    }

    #[tokio::test]
    #[serial]
    pub async fn test_pause_cyclic_process_returns_401_when_no_session_token_was_provided() {
        // Given

        let process_id = 1;

        let app = TestApp::new().await;

        // When
        let response = app
            .post(&format!("/api/cyclic-processes/{}/pause", process_id).to_string())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    #[tokio::test]
    #[serial]
    pub async fn test_pause_cyclic_process_returns_500_when_the_process_was_not_started_before() {
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

        let first_source_cyclic_process = game
            .surroundings()
            .sources()
            .first()
            .expect("Failed to get the first source of the test surroundings.")
            .process()
            .clone();

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .process_command_service
                    .assign_process_to_actors_in_game(
                        ProcessAssignmentForm::new(
                            true,
                            Vec::new(),
                            first_source_cyclic_process.id().unwrap(),
                        ),
                        &game,
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to assign the test source cyclic process.");

        // When
        let response = app
            .post(
                &format!(
                    "/api/cyclic-processes/{}/pause",
                    first_source_cyclic_process.id().unwrap()
                )
                .to_string(),
            )
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());

        let expected_error = DomainError::from(ProcessErrorKind::Pause);

        let body = read_body_as_json(response).await;
        let expected_error_message = json!({
            "code": expected_error.kind().code(),
            "message": expected_error.kind().message(),
            "context": {
                "process_id": first_source_cyclic_process.id().unwrap().to_string()
            }
        });

        assert_eq!(expected_error_message, body);
    }
}

pub mod assign_cyclic_process {
    use axum::http::StatusCode;
    use domain::{
        features::{
            actor::{domain::person::Person, dto::ActorDto},
            assignment::dto::ProcessAssignmentDto,
            auth::forms::{LoginForm, RegistrationForm},
            monk::{domain::Monk, dto::MonkDto},
            player::dto::PlayerDto,
            process::dto::{CyclicProcessDto, ProcessDto, ProcessStatusDto},
        },
        shared::{DomainElement, DurationDto},
    };
    use serde_json::json;
    use serial_test::serial;
    use time::Duration;

    use crate::shared::{
        TestApp,
        fixtures::{TestUserFixture, test_user_fixture},
        utils::read_body_as_value,
    };

    #[tokio::test]
    #[serial]
    pub async fn test_assign_process_returns_200() {
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

        let user_session = app
            .context
            .user_session_query_service
            .get_by_session_token(test_auth_tokens.session_token(), db_connection)
            .await
            .expect("Failed to get the test user session.");

        let game_id: i32 = user_session
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

        let process_id = first_source_cyclic_process.id().unwrap();

        let assigned_actors_for_process: Vec<ActorDto> = first_5_monks
            .clone()
            .into_iter()
            .map(|monk| {
                ActorDto::Monk(MonkDto {
                    id: monk.id().unwrap(),
                    name: monk.name().to_string(),
                    assigned_process_id: Some(process_id),
                    skill_ids: monk.skills().iter().map(|s| s.id().unwrap()).collect(),
                })
            })
            .chain(std::iter::once(ActorDto::Player(PlayerDto {
                id: game.player().id().unwrap(),
                assigned_process_id: Some(process_id),
            })))
            .collect();

        // When
        let response = app
            .post("/api/cyclic-processes/assign")
            .body(&json!({
                "assign_player": true,
                "actor_ids": first_5_monk_ids,
                "process_id": first_source_cyclic_process.id().unwrap()
            }))
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let expected_process_dto = CyclicProcessDto {
            id: process_id,
            cycle_interval: DurationDto::from(Duration::seconds(60)),
            elapsed: DurationDto::from(Duration::milliseconds(0)),
            paused_at: None,
            started_at: None,
            status: ProcessStatusDto::New,
            assigned_actors: assigned_actors_for_process,
        };

        let mut expected_actors: Vec<ActorDto> = first_5_monks
            .clone()
            .into_iter()
            .map(|monk| {
                ActorDto::Monk(MonkDto {
                    id: monk.id().unwrap(),
                    name: monk.name().to_string(),
                    assigned_process_id: Some(expected_process_dto.id),
                    skill_ids: monk
                        .skills()
                        .iter()
                        .map(|skill| skill.id().unwrap())
                        .collect(),
                })
            })
            .collect();

        expected_actors.push(ActorDto::Player(PlayerDto {
            id: game.player().id().unwrap(),
            assigned_process_id: Some(expected_process_dto.id),
        }));

        let expected_dto = ProcessAssignmentDto {
            actors: expected_actors,
            process: ProcessDto::CyclicProcess(expected_process_dto),
        };

        let actual_dto: ProcessAssignmentDto = read_body_as_value(response).await;

        assert_eq!(expected_dto, actual_dto);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_assign_process_returns_200_when_only_assigning_to_player() {
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

        let user_session = app
            .context
            .user_session_query_service
            .get_by_session_token(test_auth_tokens.session_token(), db_connection)
            .await
            .expect("Failed to get the test user session.");

        let game_id: i32 = user_session
            .get_game_id()
            .expect("Failed to get the test game's ID");

        let game = app
            .context
            .game_query_service
            .get_by_id(&game_id, db_connection)
            .await
            .expect("Failed to get the test game.");

        let first_source_cyclic_process = game
            .surroundings()
            .sources()
            .first()
            .expect("Failed to get the first source of the test surroundings.")
            .process()
            .clone();

        // When
        let response = app
            .post("/api/cyclic-processes/assign")
            .body(&json!({
                "assign_player": true,
                "actor_ids": [],
                "process_id": first_source_cyclic_process.id().unwrap()
            }))
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let process_id = first_source_cyclic_process.id().unwrap();

        let expected_process_dto = CyclicProcessDto {
            id: first_source_cyclic_process.id().unwrap(),
            cycle_interval: DurationDto::from(Duration::seconds(60)),
            elapsed: DurationDto::from(Duration::milliseconds(0)),
            paused_at: None,
            started_at: None,
            status: ProcessStatusDto::New,
            assigned_actors: vec![ActorDto::Player(PlayerDto {
                id: game.player().id().unwrap(),
                assigned_process_id: Some(process_id),
            })],
        };

        let expected_dto = ProcessAssignmentDto {
            actors: vec![ActorDto::Player(PlayerDto {
                id: game.player().id().unwrap(),
                assigned_process_id: Some(process_id),
            })],
            process: ProcessDto::CyclicProcess(expected_process_dto),
        };

        let actual_dto: ProcessAssignmentDto = read_body_as_value(response).await;

        assert_eq!(expected_dto, actual_dto);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_assign_process_returns_200_when_only_assigning_to_monks() {
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

        // When
        let response = app
            .post("/api/cyclic-processes/assign")
            .body(&json!({
                "assign_player": false,
                "actor_ids": first_5_monk_ids,
                "process_id": first_source_cyclic_process.id().unwrap()
            }))
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let process_id = first_source_cyclic_process.id().unwrap();

        let assigned_actors_for_process: Vec<ActorDto> = first_5_monks
            .clone()
            .into_iter()
            .map(|monk| {
                ActorDto::Monk(MonkDto {
                    id: monk.id().unwrap(),
                    name: monk.name().to_string(),
                    assigned_process_id: Some(process_id),
                    skill_ids: monk.skills().iter().map(|s| s.id().unwrap()).collect(),
                })
            })
            .collect();

        let expected_process_dto = ProcessDto::CyclicProcess(CyclicProcessDto {
            id: first_source_cyclic_process.id().unwrap(),
            cycle_interval: DurationDto::from(Duration::seconds(60)),
            elapsed: DurationDto::from(Duration::milliseconds(0)),
            paused_at: None,
            started_at: None,
            status: ProcessStatusDto::New,
            assigned_actors: assigned_actors_for_process.clone(),
        });

        let expected_dto = ProcessAssignmentDto {
            actors: assigned_actors_for_process,
            process: expected_process_dto,
        };

        let actual_dto: ProcessAssignmentDto = read_body_as_value(response).await;

        assert_eq!(expected_dto, actual_dto);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_assign_process_returns_200_without_actor_ids() {
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

        let first_source_cyclic_process = game
            .surroundings()
            .sources()
            .first()
            .expect("Failed to get the first source of the test surroundings.")
            .process()
            .clone();

        // When
        let response = app
            .post("/api/cyclic-processes/assign")
            .body(&json!({
                "assign_player": false,
                "actor_ids": [],
                "process_id": first_source_cyclic_process.id().unwrap()
            }))
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let expected_process_dto = ProcessDto::CyclicProcess(CyclicProcessDto {
            id: first_source_cyclic_process.id().unwrap(),
            cycle_interval: DurationDto::from(Duration::seconds(60)),
            elapsed: DurationDto::from(Duration::milliseconds(0)),
            paused_at: None,
            started_at: None,
            status: ProcessStatusDto::New,
            assigned_actors: Vec::new(),
        });

        let expected_dto = ProcessAssignmentDto {
            actors: Vec::new(),
            process: expected_process_dto,
        };

        let actual_dto: ProcessAssignmentDto = read_body_as_value(response).await;

        assert_eq!(expected_dto, actual_dto);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_assign_process_returns_500_with_an_invalid_process_id() {
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
            .post("/api/cyclic-processes/assign")
            .body(&json!({
                "assign_player": false,
                "actor_ids": [],
                "process_id": 0
            }))
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
    }

    #[tokio::test]
    #[serial]
    pub async fn test_assign_process_returns_422_with_an_invalid_request_form() {
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
            .post("/api/cyclic-processes/assign")
            .body(&json!({
                "actor_ids": [],
                "process_id": ""
            }))
            .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, response.status());
    }
}

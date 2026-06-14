pub mod register {
    use axum::http::StatusCode;
    use domain::{
        features::auth::error::{AuthenticationErrorKind, RegistrationErrorKind},
        shared::error::DomainErrorKind,
    };
    use serde_json::json;
    use serial_test::serial;

    use crate::shared::{
        TestApp,
        fixtures::{EXAMPLE_GAME_SEED, TestUserFixture, test_user_fixture},
        utils::read_body_as_json,
    };

    #[tokio::test]
    #[serial]
    pub async fn test_register_new_user_returns_200() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();
        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_register_new_user_returns_200_after_multiple_user_registrations() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();
        let app = TestApp::new().await;

        app.post("/api/auth/register")
            .body(&json!({
                "email": "test@test.test",
                "password": "test",
                "confirmed_password": "test"
            }))
            .send()
            .await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_register_new_user_returns_200_when_providing_game_options() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();
        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password,
                "game_options": {
                    "game_seed": EXAMPLE_GAME_SEED
                }
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_register_new_user_returns_200_when_providing_partial_game_options() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();
        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password,
                "game_options": {}
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_register_new_user_returns_422_when_providing_wrong_parameters() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();
        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password,
                "test_options": {}
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_register_new_user_returns_400_when_the_password_is_not_confirmed() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();
        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": "test"
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_register_new_user_returns_500_when_an_email_address_is_already_registered() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password
            }))
            .send()
            .await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let expected_error_kind =
            AuthenticationErrorKind::Registration(RegistrationErrorKind::EmailAlreadyExists);

        let expected_json = json!({
            "code": expected_error_kind.code(),
            "message": expected_error_kind.message(),
            "context": json!({
                "email": email
            })
        });

        let actual_json: serde_json::Value = read_body_as_json(response).await;

        assert_eq!(actual_json, expected_json);
    }

    #[tokio::test]
    async fn test_register_users_returns_500_when_the_form_contains_unknown_fields() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "email": email,
                "password": password,
                "confirmed_password": password,
                "phone_number": "123456789"
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_register_new_user_returns_400_when_the_form_contains_no_email_address() {
        // Given
        let TestUserFixture { password, .. } = test_user_fixture();

        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/register")
            .body(&json!({
                "password": password,
                "confirmed_password": password
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

pub mod login {
    use axum::http::StatusCode;
    use domain::features::{
        auth::{domain::AuthenticationTokens, forms::UserRegistrationForm},
        game::dto::GameOptionsForm,
    };
    use serde_json::json;
    use serial_test::serial;
    use tracing::debug;

    use crate::shared::{
        TestApp,
        fixtures::{TestUserFixture, test_user_fixture},
        utils::{extract_cookies, read_body_as_json},
    };

    #[tokio::test]
    #[serial]
    pub async fn test_login_returns_200() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .register(
                        UserRegistrationForm::new(&email, &password),
                        GameOptionsForm::empty(),
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to register the test user.");

        // When
        let response = app
            .post("/api/auth/login")
            .body(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let cookies = extract_cookies(&response);

        assert!(cookies.contains_key(&AuthenticationTokens::RefreshToken.cookie_name()));

        let body: serde_json::value::Value = read_body_as_json(response).await;

        assert_ne!(
            "",
            body.get(AuthenticationTokens::SessionToken.cookie_name())
                .unwrap()
        );
    }

    #[tokio::test]
    #[serial]
    pub async fn test_login_returns_200_after_logging_in_multiple_times() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .register(
                        UserRegistrationForm::new(&email, &password),
                        GameOptionsForm::empty(),
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to register the test user.");

        app.post("/api/auth/login")
            .body(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await;

        // When
        debug!("Test log-in attempt");

        let response = app
            .post("/api/auth/login")
            .body(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    #[serial]
    pub async fn test_login_user_returns_401_with_wrong_email() {
        // Given
        let TestUserFixture { password, .. } = test_user_fixture();

        let app = TestApp::new().await;

        // When
        let response = app
            .post("/api/auth/login")
            .body(&json!({
                "email": "test@test.example",
                "password": password
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_login_user_returns_401_with_invalid_password() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .register(
                        UserRegistrationForm::new(&email, &password),
                        GameOptionsForm::empty(),
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to register the test user.");

        // When
        let response = app
            .post("/api/auth/login")
            .body(&json!({
                "email": email,
                "password": "wrongPassword"
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_login_user_returns_400_with_invalid_login_form() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .register(
                        UserRegistrationForm::new(&email, &password),
                        GameOptionsForm::empty(),
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to register the test user.");

        // When
        let response = app
            .post("/api/auth/login")
            .body(&json!({
                "email": email
            }))
            .send()
            .await;

        // Then
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

pub mod refresh {
    use axum::http::StatusCode;
    use domain::{
        features::{
            auth::{
                domain::AuthenticationTokens,
                error::{AuthenticationErrorKind, RefreshErrorKind},
                forms::{UserLoginForm, UserRegistrationForm},
            },
            game::dto::GameOptionsForm,
        },
        shared::error::{DomainError, DomainErrorKind},
    };
    use serde_json::json;
    use serial_test::serial;

    use crate::shared::{
        TestApp,
        fixtures::{TestUserFixture, test_user_fixture},
        utils::{extract_cookies, read_body, read_body_as_json},
    };

    #[tokio::test]
    #[serial]
    pub async fn test_refresh_returns_200() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .register(
                        UserRegistrationForm::new(&email, &password),
                        GameOptionsForm::empty(),
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to register the test user.");

        let test_auth_tokens = app
            .context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .login(UserLoginForm::new(email, password), db_transaction)
                    .await
            })
            .await
            .expect("Failed to login the test user.");

        // When
        let response = app
            .post("/api/auth/refresh")
            .cookie(
                AuthenticationTokens::RefreshToken.cookie_name(),
                test_auth_tokens.refresh_token().value().to_string(),
            )
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::OK, response.status());

        let cookies = extract_cookies(&response);

        assert!(cookies.contains_key(&AuthenticationTokens::RefreshToken.cookie_name()));
        assert_ne!(
            *test_auth_tokens.refresh_token().value().to_string(),
            cookies
                .get(&AuthenticationTokens::RefreshToken.cookie_name())
                .unwrap()
                .to_string()
        );

        let body = read_body_as_json(response).await;

        assert_ne!(
            test_auth_tokens
                .session_token()
                .to_jwt()
                .expect("The previous JWT to exist.")
                .to_string(),
            body.get(AuthenticationTokens::SessionToken.cookie_name())
                .unwrap()
                .to_string()
        );
    }

    #[tokio::test]
    #[serial]
    pub async fn test_refresh_returns_401_when_no_refresh_token_was_provided() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .register(
                        UserRegistrationForm::new(&email, &password),
                        GameOptionsForm::empty(),
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to register the test user.");

        // When
        let response = app.post("/api/auth/refresh").send().await;

        // Then
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let cookies = extract_cookies(&response);
        assert!(!cookies.contains_key(&AuthenticationTokens::RefreshToken.cookie_name()));

        let body = read_body(response).await;
        assert_eq!(String::new(), body);
    }

    #[tokio::test]
    #[serial]
    pub async fn test_refresh_returns_401_when_a_invalid_token_was_provided() {
        // Given
        let TestUserFixture { email, password } = test_user_fixture();

        let app = TestApp::new().await;

        app.context
            .in_transaction(async |db_transaction| {
                app.context
                    .authentication_service
                    .register(
                        UserRegistrationForm::new(&email, &password),
                        GameOptionsForm::empty(),
                        db_transaction,
                    )
                    .await
            })
            .await
            .expect("Failed to register the test user.");

        // When
        let response = app
            .post("/api/auth/refresh")
            .cookie(AuthenticationTokens::RefreshToken.cookie_name(), "TEST")
            .send()
            .await;

        // Then
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());

        let cookies = extract_cookies(&response);
        assert!(!cookies.contains_key(&AuthenticationTokens::RefreshToken.cookie_name()));

        let expected_error = DomainError::from(AuthenticationErrorKind::Refresh(
            RefreshErrorKind::RefreshTokenNotFound,
        ));
        let body = read_body_as_json(response).await;
        let expected_error_message = json!({
            "code": expected_error.kind().code(),
            "message": expected_error.kind().message()
        });

        assert_eq!(expected_error_message, body);
    }
}

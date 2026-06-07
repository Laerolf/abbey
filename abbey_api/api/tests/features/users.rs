use axum::http::StatusCode;
use domain::{
    features::{
        auth::forms::{UserLoginForm, UserRegistrationForm},
        user::dto::UserDto,
    },
    shared::DomainElement,
};

use serial_test::serial;

use crate::shared::{
    TestApp,
    fixtures::{TestUserFixture, test_user_fixture},
    utils::read_body_as_value,
};

#[tokio::test]
#[serial]
pub async fn test_get_session_user_returns_200() {
    // Given
    let TestUserFixture { email, password } = test_user_fixture();

    let app = TestApp::new().await;

    let test_user = app
        .context
        .in_transaction(async |db_transaction| {
            app.context
                .authentication_service
                .register(UserRegistrationForm::new(&email, &password), db_transaction)
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
        .get("/api/users/me")
        .bearer(test_auth_tokens.session_token().to_jwt().unwrap())
        .send()
        .await;

    // Then
    assert_eq!(StatusCode::OK, response.status());

    let expected_dto = UserDto {
        id: test_user.id().unwrap(),
        email: test_user.email().to_string(),
    };

    let actual_dto: UserDto = read_body_as_value(response).await;

    assert_eq!(expected_dto, actual_dto)
}

#[tokio::test]
#[serial]
pub async fn test_get_session_user_returns_401_when_not_authenticated() {
    // Given
    let app = TestApp::new().await;

    // When
    let response = app.get("/api/users/me").send().await;

    // Then
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

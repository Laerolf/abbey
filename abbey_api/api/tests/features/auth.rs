use axum::http::StatusCode;
use serde_json::json;
use tracing::info;

use crate::shared::TestApp;

#[tokio::test]
async fn test_create_user_returns_200() {
    let app = TestApp::instance().await;
    app.reset_database().await;

    let response = app
        .post(
            "/api/auth/register",
            json!({
                "email": "test@example.com"
            }),
        )
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user_returns_500_when_the_email_is_already_registered() {
    let app = TestApp::instance().await;
    app.reset_database().await;

    app.post(
        "/api/auth/register",
        json!({
            "email": "test@example.com"
        }),
    )
    .await;

    let response = app
        .post(
            "/api/auth/register",
            json!({
                "email": "test@example.com"
            }),
        )
        .await;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    info!("{:?}", response.body());
}

use api::features::openapi;
use axum::http::StatusCode;
use serde_json::json;
use serial_test::serial;

use crate::shared::{TestApp, utils::read_body_as_json};

#[tokio::test]
#[serial]
pub async fn test_swagger_openapi_returns_303() {
    // Given
    let app = TestApp::new().await;

    // When
    let response = app.get("/openapi").send().await;

    // Then
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
#[serial]
pub async fn test_openapi_json_returns_200() {
    // Given
    let app = TestApp::new().await;

    // When
    let response = app.get("/openapi.json").send().await;

    // Then
    assert_eq!(response.status(), StatusCode::OK);

    let expected_openapi_json = json!(openapi());
    let actual_openapi_json = read_body_as_json(response).await;

    assert_eq!(expected_openapi_json, actual_openapi_json);
}

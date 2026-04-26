use std::collections::HashMap;

use axum::{
    body::{Body, to_bytes},
    http::{Response, header},
};
use serde::de::DeserializeOwned;

use tower_cookies::Cookie;

/// Converts the body of a response to a string.
pub async fn read_body(response: Response<Body>) -> String {
    let body_bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");

    String::from_utf8(body_bytes.to_vec()).expect("Response body was not valid UTF-8")
}

/// Converts the body of a response to an element.
pub async fn read_body_as_value<T: DeserializeOwned>(response: Response<Body>) -> T {
    serde_json::from_value(read_body_as_json(response).await)
        .expect("Failed to deserialize the response body as an element.")
}

/// Converts the body of a response to a JSON object.
pub async fn read_body_as_json(response: Response<Body>) -> serde_json::Value {
    serde_json::from_str(&read_body(response).await)
        .expect("Failed to deserialize response body as JSON")
}

/// Extracts cookes from a response.
pub fn extract_cookies(response: &Response<Body>) -> HashMap<String, Cookie<'static>> {
    let mut cookies = HashMap::new();

    for cookie_header in response.headers().get_all(header::SET_COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str()
            && let Ok(cookie) = Cookie::parse(cookie_str)
        {
            cookies.insert(cookie.name().to_string(), cookie.into_owned());
        }
    }

    cookies
}

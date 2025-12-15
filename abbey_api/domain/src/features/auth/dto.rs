use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: i32,
    pub email: String,
}

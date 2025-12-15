use serde::Serialize;

#[derive(Serialize)]
pub struct UserDto {
    pub user_id: i32,
    pub email: String,
}

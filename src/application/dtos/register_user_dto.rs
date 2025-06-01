use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserResponseDto {
    pub id: Uuid,
    pub email: String,
}

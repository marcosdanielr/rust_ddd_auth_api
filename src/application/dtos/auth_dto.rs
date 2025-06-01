#[derive(serde::Deserialize)]
pub struct AuthRequestDto {
    pub email: String,
    pub password: String,
}

pub struct AuthResponseDto {
    pub access_token: String,
}

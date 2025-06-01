use std::f32::consts::E;

use crate::{
    application::dtos::auth_dto::{AuthRequestDto, AuthResponseDto},
    domain::repositories::user_repository::UserRepository,
    infra::security::{jwt::JwtService, password_hasher::PasswordHasher},
};

pub struct AuthenticateUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> AuthenticateUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self, dto: AuthRequestDto) -> Result<AuthResponseDto, String> {
        let user = match self.user_repository.find_by_email(&dto.email) {
            Some(user) => user,
            None => return Err("Invalid credentials".to_string()),
        };

        let is_valid_password =
            PasswordHasher::verify_password(&dto.password, &user.password_hash())?;

        if !is_valid_password {
            return Err("Invalid credentials".to_string());
        }

        let token = JwtService::generate_token(user.id().to_string().as_str())?;

        Ok(AuthResponseDto {
            access_token: token,
        })
    }
}

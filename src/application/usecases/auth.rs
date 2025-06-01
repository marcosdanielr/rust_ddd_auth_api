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

    pub async fn execute(&self, dto: AuthRequestDto) -> Result<AuthResponseDto, String> {
        let user = match self.user_repository.find_by_email(&dto.email).await {
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

#[cfg(test)]
mod tests {
    use crate::{
        domain::entities::user::User,
        infra::repositories::in_memory_user_repository::InMemoryUserRepository,
    };

    use super::*;

    #[tokio::test]
    async fn test_authenticate_user_use_case_success() {
        let user_repository = InMemoryUserRepository::new();

        let sut = AuthenticateUseCase::new(&user_repository);

        let user = User::new(
            "test@example.com".to_string(),
            PasswordHasher::hash_password("password").unwrap(),
        );

        user_repository.create(&user).await.unwrap();

        let auth_request = AuthRequestDto {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        let result = sut.execute(auth_request).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.access_token.is_empty());
    }

    #[tokio::test]
    async fn test_authenticate_user_use_case_invalid_credentials() {
        let user_repository = InMemoryUserRepository::new();

        let user = User::new(
            "test@example.com".to_string(),
            PasswordHasher::hash_password("password").unwrap(),
        );

        user_repository.create(&user).await.unwrap();

        let sut = AuthenticateUseCase::new(&user_repository);

        let mut auth_request = AuthRequestDto {
            email: "test@example.com".to_string(),
            password: "wrong-password".to_string(),
        };

        let mut result = sut.execute(auth_request).await;

        assert!(result.is_err());

        auth_request = AuthRequestDto {
            email: "wrong_email@example.com".to_string(),
            password: "password".to_string(),
        };

        result = sut.execute(auth_request).await;

        assert!(result.is_err());
    }
}

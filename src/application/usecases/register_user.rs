use crate::{
    application::dtos::register_user_dto::{RegisterUserRequestDto, RegisterUserResponseDto},
    domain::{entities::user::User, repositories::user_repository::UserRepository},
    infra::security::password_hasher::PasswordHasher,
};

use super::errors::user_error::RegisterUserError;

pub struct RegisterUserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> RegisterUserUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        data: RegisterUserRequestDto,
    ) -> Result<RegisterUserResponseDto, RegisterUserError> {
        if !User::validate_email(&data.email) {
            return Err(RegisterUserError::InvalidEmail);
        }

        if !User::validate_password(&data.password) {
            return Err(RegisterUserError::PasswordShort);
        }

        let user_exists = self.user_repository.find_by_email(&data.email).await;

        if user_exists.is_some() {
            return Err(RegisterUserError::UserExists);
        }

        let password_hashed = PasswordHasher::hash_password(&data.password)
            .map_err(|_| RegisterUserError::Unknown)?;

        let new_user_data = User::new(data.email, password_hashed);

        self.user_repository
            .create(&new_user_data)
            .await
            .map_err(|_| RegisterUserError::Unknown)?;

        Ok(RegisterUserResponseDto {
            id: new_user_data.id().clone(),
            email: new_user_data.email().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::dtos::register_user_dto::RegisterUserRequestDto,
        infra::database::repositories::in_memory_user_repository::InMemoryUserRepository,
    };

    #[tokio::test]
    async fn test_register_user_use_case_success() {
        let user_repository = InMemoryUserRepository::new();
        let sut = RegisterUserUseCase::new(&user_repository);

        let dto = RegisterUserRequestDto {
            email: "test@example.com".to_string(),
            password: "12345678".to_string(),
        };

        let result = sut.execute(dto).await;

        assert!(result.is_ok());
        let user = result.unwrap();

        let stored_user = user_repository.find_by_email(&user.email).await;
        assert!(stored_user.is_some());

        assert_eq!(stored_user.unwrap().email(), user.email);
    }

    #[tokio::test]
    async fn test_register_user_use_case_password_too_short() {
        let user_repo = InMemoryUserRepository::new();
        let sut = RegisterUserUseCase::new(&user_repo);

        let dto = RegisterUserRequestDto {
            email: "test@example.com".to_string(),
            password: "1234".to_string(),
        };

        let result = sut.execute(dto).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            RegisterUserError::PasswordShort.to_string()
        );
    }

    #[tokio::test]
    async fn test_register_user_use_case_email_exists() {
        let user_repo = InMemoryUserRepository::new();
        let sut = RegisterUserUseCase::new(&user_repo);

        let result = sut
            .execute(RegisterUserRequestDto {
                email: "test@example.com".to_string(),
                password: "123456789".to_string(),
            })
            .await;

        assert!(result.is_ok());

        let result = sut
            .execute(RegisterUserRequestDto {
                email: "test@example.com".to_string(),
                password: "2342343245".to_string(),
            })
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            RegisterUserError::UserExists.to_string()
        );
    }
}

use crate::{
    application::dtos::register_user_dto::RegisterUserDto,
    domain::{entities::user::User, repositories::user_repository::UserRepository},
    infra::security::password_hasher::PasswordHasher,
};

pub struct RegisterUserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> RegisterUserUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, data: RegisterUserDto) -> Result<User, String> {
        if !User::validate_email(&data.email) {
            return Err("Invalid email".to_string());
        }

        if !User::validate_password(&data.password) {
            return Err("Password to short".into());
        }

        let password_hashed = PasswordHasher::hash_password(&data.password)
            .map_err(|_| "Failed to hash password".to_string())?;

        let new_user_data = User::new(data.email, password_hashed);

        self.user_repository
            .create(&new_user_data)
            .await
            .map_err(|_| "Failed to save user".to_string())?;

        Ok(new_user_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::dtos::register_user_dto::RegisterUserDto,
        infra::repositories::in_memory_user_repository::InMemoryUserRepository,
    };

    #[tokio::test]
    async fn test_register_user_use_case_success() {
        let user_repository = InMemoryUserRepository::new();
        let sut = RegisterUserUseCase::new(&user_repository);

        let dto = RegisterUserDto {
            email: "test@example.com".to_string(),
            password: "12345678".to_string(),
        };

        let result = sut.execute(dto).await;

        assert!(result.is_ok());
        let user = result.unwrap();

        let stored_user = user_repository.find_by_email(&user.email()).await;
        assert!(stored_user.is_some());

        assert_eq!(stored_user.unwrap().email(), user.email());
    }

    #[tokio::test]
    async fn test_register_user_use_case_password_too_short() {
        let user_repo = InMemoryUserRepository::new();
        let sut = RegisterUserUseCase::new(&user_repo);

        let dto = RegisterUserDto {
            email: "test@example.com".to_string(),
            password: "1234".to_string(),
        };

        let result = sut.execute(dto).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Password too short".to_string());
    }
}

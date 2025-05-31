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

    pub fn execute(&self, data: RegisterUserDto) -> Result<(), String> {
        if !User::validate_email(&data.email) {
            return Err("Invalid email".to_string());
        }

        if !User::validate_password(&data.password) {
            return Err("Password to short".into());
        }

        let password_hashed = PasswordHasher::hash_password(&data.password)
            .map_err(|_| "Failed to hash password".to_string())?;

        let new_user_data = User::new(data.email, password_hashed);

        let _ = self
            .user_repository
            .create(&new_user_data)
            .map_err(|e| "Failed to save user");

        Ok(())
    }
}

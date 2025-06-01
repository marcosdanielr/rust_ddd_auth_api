use crate::domain::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<(), String>;
    async fn find_by_email(&self, email: &str) -> Option<User>;
}

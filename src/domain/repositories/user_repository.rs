use crate::domain::entities::user::User;

pub trait UserRepository {
    fn create(&self, user: &User) -> Result<(), String>;
    fn find_by_email(&self, email: &str) -> Option<User>;
}

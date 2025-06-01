use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;

#[derive(Default)]
pub struct InMemoryUserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

impl InMemoryUserRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(vec![])),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, user: &User) -> Result<(), String> {
        let mut users = self.users.lock().map_err(|_| "Mutex poisoned")?;
        users.push(user.clone());
        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        let users = self.users.lock().ok()?;
        users.iter().find(|user| user.email() == email).cloned()
    }
}

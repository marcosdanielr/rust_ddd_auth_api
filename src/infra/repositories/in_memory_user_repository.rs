use std::{cell::RefCell, rc::Rc};

use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;

#[derive(Default)]
pub struct InMemoryUserRepository {
    users: Rc<RefCell<Vec<User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn create(&self, user: &User) -> Result<(), String> {
        let mut users = self.users.borrow_mut();
        users.push(user.clone());
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Option<User> {
        let users = self.users.borrow();
        users.iter().find(|user| user.email() == email).cloned()
    }
}

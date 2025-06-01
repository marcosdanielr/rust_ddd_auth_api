use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use sea_orm::{DbBackend, EntityTrait, Statement};

use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};

use crate::infra::database::entities::user_entity as user;

#[derive(Default)]
pub struct SeaORMUserRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaORMUserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaORMUserRepository {
    async fn create(&self, user: &User) -> Result<(), String> {
        let user_model = user::ActiveModel {
            id: Set(user.id().clone()),
            email: Set(user.email().to_string()),
            password_hash: Set(user.password_hash().to_string()),
        };

        user_model
            .insert(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        let user_model: Option<user::Model> = user::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"SELECT * FROM "users" WHERE "email" = $1"#,
                [email.into()],
            ))
            .one(self.db.as_ref())
            .await
            .ok()?;

        user_model.map(|u| User::new_with_id(u.id, u.email, u.password_hash))
    }
}

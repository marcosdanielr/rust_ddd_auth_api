use axum::{Json, extract::Extension};

use crate::application::http::middlewares::auth_middleware::AuthenticatedUser;

pub async fn me_handler(Extension(user): Extension<AuthenticatedUser>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "user_id": user.user_id
    }))
}

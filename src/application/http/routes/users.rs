use axum::{Router, routing::post};

use crate::{application::http::handlers::register_user::register_user_handler, state::AppState};

pub fn users_routes() -> Router<AppState> {
    Router::new().route("/", post(register_user_handler))
}

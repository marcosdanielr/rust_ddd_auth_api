use axum::{Router, routing::post};

use crate::{application::http::handlers::auth_handler::auth_handler, state::AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/", post(auth_handler))
}

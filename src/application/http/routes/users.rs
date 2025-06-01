use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    application::http::{
        handlers::{me_handler::me_handler, register_user::register_user_handler},
        middlewares::auth_middleware::auth_middleware,
    },
    state::AppState,
};

pub fn users_routes() -> Router<AppState> {
    Router::new().route("/", post(register_user_handler)).route(
        "/me",
        get(me_handler).route_layer(middleware::from_fn(auth_middleware)),
    )
}

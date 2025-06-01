mod application;
mod domain;
mod infra;

mod state;

use std::env;

use application::http::routes::{auth::auth_routes, users::users_routes};
use dotenvy::dotenv;
use infra::database::connection::establish_db_connection;

use axum::Router;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "4000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let db = establish_db_connection().await;
    let app_state = AppState { db: db.into() };

    let app = Router::new()
        .nest("/api/auth", auth_routes().with_state(app_state.clone()))
        .nest("/api/users", users_routes().with_state(app_state.clone()));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

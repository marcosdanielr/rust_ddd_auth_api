mod application;
mod domain;
mod infra;

mod state;

use application::http::routes::auth::auth_routes;
use infra::database::connection::establish_db_connection;

use axum::Router;
use state::AppState;

#[tokio::main]
async fn main() {
    let db = establish_db_connection().await;

    let app_state = AppState { db: db.into() };

    let app = Router::new().nest("/api/auth", auth_routes().with_state(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

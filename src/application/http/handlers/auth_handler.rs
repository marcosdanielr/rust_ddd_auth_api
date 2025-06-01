use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{
    application::{
        dtos::auth_dto::AuthRequestDto,
        usecases::{auth::AuthenticateUseCase, errors::auth_error::AuthError},
    },
    infra::database::repositories::seaorm_user_repository::SeaORMUserRepository,
    state::AppState,
};

#[axum::debug_handler]
pub async fn auth_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequestDto>,
) -> impl IntoResponse {
    let user_repo = SeaORMUserRepository::new(state.db.clone());
    let auth_use_case = AuthenticateUseCase::new(&user_repo);

    match auth_use_case.execute(payload).await {
        Ok(auth_response) => (
            StatusCode::OK,
            Json(json!({ "access_token": auth_response.access_token })),
        )
            .into_response(),

        Err(AuthError::InvalidCredentials) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid credentials" })),
        )
            .into_response(),

        Err(AuthError::Unknown) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Internal server error" })),
        )
            .into_response(),
    }
}

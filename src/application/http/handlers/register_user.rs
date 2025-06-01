use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    application::{
        dtos::register_user_dto::RegisterUserRequestDto,
        usecases::{errors::user_error::RegisterUserError, register_user::RegisterUserUseCase},
    },
    infra::database::repositories::seaorm_user_repository::SeaORMUserRepository,
    state::AppState,
};

#[axum::debug_handler]
pub async fn register_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequestDto>,
) -> impl IntoResponse {
    let user_repo = SeaORMUserRepository::new(state.db.clone());
    let register_user_use_case = RegisterUserUseCase::new(&user_repo);

    match register_user_use_case.execute(payload).await {
        Ok(user_created) => (StatusCode::CREATED, Json(user_created)).into_response(),

        Err(RegisterUserError::UserExists) => (
            StatusCode::CONFLICT,
            Json(json!({ "message": "User already exists!" })),
        )
            .into_response(),

        Err(RegisterUserError::PasswordShort) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Password too short" })),
        )
            .into_response(),
        Err(RegisterUserError::InvalidEmail) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Invalid email format" })),
        )
            .into_response(),
        Err(RegisterUserError::Unknown) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Internal server error" })),
        )
            .into_response(),
    }
}

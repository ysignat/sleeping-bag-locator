use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse};

use super::state::AppState;
use crate::http::common::AppError;

#[debug_handler]
pub async fn health(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    state.items.health().await?;
    state.users.health().await?;

    Ok(StatusCode::OK)
}

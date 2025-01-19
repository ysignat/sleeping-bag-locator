use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use super::dtos::{HttpCreateItemParams, HttpItem, HttpUpdateItemParams};
use crate::{
    dao::Pagination,
    http::{
        common::{AppError, HttpPaginationParams},
        AppState,
    },
};

#[debug_handler]
pub async fn list_items(
    Query(pagination_params): Query<HttpPaginationParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let pagination: Pagination = pagination_params.try_into()?;
    let response_headers: HeaderMap = pagination.clone().try_into()?;
    let result: Vec<HttpItem> = state
        .items
        .list(pagination)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, response_headers, Json(result)))
}

#[debug_handler]
pub async fn create_item(
    State(state): State<AppState>,
    Json(params): Json<HttpCreateItemParams>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpItem = state.items.create(params.try_into()?).await?.into();

    Ok((StatusCode::CREATED, Json(result)))
}

#[debug_handler]
pub async fn get_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpItem = state.items.get(id).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

#[debug_handler]
pub async fn update_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(params): Json<HttpUpdateItemParams>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpItem = state.items.update(id, params.try_into()?).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

#[debug_handler]
pub async fn delete_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    state.items.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn health(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    state.items.health().await?;

    Ok(StatusCode::OK)
}

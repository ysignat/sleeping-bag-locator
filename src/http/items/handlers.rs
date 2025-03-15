use axum::{
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

pub async fn list_items<T>(
    Query(pagination_params): Query<HttpPaginationParams>,
    State(state): State<AppState<T>>,
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

pub async fn create_item<T>(
    State(state): State<AppState<T>>,
    Json(params): Json<HttpCreateItemParams>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpItem = state.items.create(params.try_into()?).await?.into();

    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn get_item<T>(
    Path(id): Path<Uuid>,
    State(state): State<AppState<T>>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpItem = state.items.get(id).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

pub async fn update_item<T>(
    Path(id): Path<Uuid>,
    State(state): State<AppState<T>>,
    Json(params): Json<HttpUpdateItemParams>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpItem = state.items.update(id, params.try_into()?).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

pub async fn delete_item<T>(
    Path(id): Path<Uuid>,
    State(state): State<AppState<T>>,
) -> Result<impl IntoResponse, AppError> {
    state.items.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn health<T>(State(state): State<AppState<T>>) -> Result<impl IntoResponse, AppError> {
    state.items.health().await?;

    Ok(StatusCode::OK)
}

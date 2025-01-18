use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use super::{
    errors::AppError,
    AppState,
    Entity,
    Limit,
    MutableParams,
    Page,
    PaginationParams,
    Params,
};

#[debug_handler]
pub async fn list(
    Query(pagination_params): Query<PaginationParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let pagination = pagination_params.clone();
    let mut headers = HeaderMap::new();

    if let Some(page) = pagination_params.page {
        headers.insert(Page::HEADER, page.try_into()?);
    }

    if let Some(limit) = pagination_params.limit {
        headers.insert(Limit::HEADER, limit.try_into()?);
    }

    let result: Vec<Entity> = state
        .list(pagination.try_into()?)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, headers, Json(result)))
}

#[debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Json(params): Json<Params>,
) -> Result<impl IntoResponse, AppError> {
    let result: Entity = state.create(params.try_into()?).await?.into();

    Ok((StatusCode::CREATED, Json(result)))
}

#[debug_handler]
pub async fn get(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result: Entity = state.get(id).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

#[debug_handler]
pub async fn update(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(mutable_params): Json<MutableParams>,
) -> Result<impl IntoResponse, AppError> {
    let result: Entity = state.update(id, mutable_params.try_into()?).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

#[debug_handler]
pub async fn delete(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    state.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn health(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    state.health().await?;

    Ok(StatusCode::OK)
}

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Json,
    Router,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct Item {
    id: Uuid,
    name: String,
    location: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
struct UpdateItemBody {
    name: String,
    location: String,
}

#[derive(Debug, Deserialize)]
struct ListQueryParams {
    page: Option<i32>,
    limit: Option<i32>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        );

    axum::serve(listener, router).await.unwrap();
}

async fn list_items(Query(list_params): Query<ListQueryParams>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Pagination-Page",
        HeaderValue::from_str(&list_params.page.unwrap_or(1).to_string()).unwrap(),
    );
    headers.insert(
        "Pagination-Limit",
        HeaderValue::from_str(&list_params.limit.unwrap_or(10).to_string()).unwrap(),
    );

    (
        StatusCode::OK,
        headers,
        Json(vec![Item {
            id: Uuid::new_v4(),
            name: "Sleeping Bag".to_owned(),
            location: "Calgary, AB".to_owned(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }]),
    )
}

async fn create_item(Json(body): Json<UpdateItemBody>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(Item {
            id: Uuid::new_v4(),
            name: body.name,
            location: body.location,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }),
    )
}

async fn get_item(Query(id): Query<Uuid>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Item {
            id,
            name: "Sleeping Bag".to_owned(),
            location: "Calgary, AB".to_owned(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }),
    )
}

async fn update_item(
    Query(id): Query<Uuid>,
    Json(body): Json<UpdateItemBody>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Item {
            id,
            name: body.name,
            location: body.location,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }),
    )
}

async fn delete_item(Query(_): Query<Uuid>) -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

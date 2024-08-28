#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use anyhow::Context;
use axum::{
    extract::Query,
    http::{header::InvalidHeaderValue, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Json,
    Router,
};
use chrono::{NaiveDateTime, Utc};
use errors::AppError;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use uuid::Uuid;

mod errors;

const BIND_ADDRESS: &str = "0.0.0.0:8080";
const PAGINATION_LIMIT_HEADER: &str = "Pagination-Limit";
const DEFAULT_PAGINATION_LIMIT: u32 = 10;
const PAGINATION_PAGE_HEADER: &str = "Pagination-Page";
const DEFAULT_PAGINATION_PAGE: u32 = 1;
const MAX_ITEM_NAME_LENGTH: usize = 128;
const MAX_ITEM_LOCATION_LENGTH: usize = 128;

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "String")]
struct ItemName(String);

impl TryFrom<String> for ItemName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Empty string cannot be name of the item".to_owned())
        } else if value.len().gt(&MAX_ITEM_NAME_LENGTH) {
            Err(format!(
                "Item name cannot be longer than {MAX_ITEM_NAME_LENGTH}"
            ))
        } else {
            Ok(ItemName(value))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "String")]
struct ItemLocation(String);

impl TryFrom<String> for ItemLocation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Empty string cannot be location of the item".to_owned())
        } else if value.len().gt(&MAX_ITEM_LOCATION_LENGTH) {
            Err(format!(
                "Item location cannot be longer than {MAX_ITEM_LOCATION_LENGTH}"
            ))
        } else {
            Ok(ItemLocation(value))
        }
    }
}

#[derive(Debug, Serialize)]
struct Item {
    id: Uuid,
    name: ItemName,
    location: ItemLocation,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
struct UpdateItemBody {
    name: ItemName,
    location: ItemLocation,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "Option<u32>")]
struct Page(u32);

impl TryFrom<Option<u32>> for Page {
    type Error = String;

    fn try_from(value: Option<u32>) -> Result<Self, Self::Error> {
        match value {
            Some(page) => {
                if page.eq(&0) {
                    Err("Page cannot be equal zero".to_owned())
                } else {
                    Ok(Page(page))
                }
            }
            None => Ok(Page(DEFAULT_PAGINATION_PAGE)),
        }
    }
}

impl TryInto<HeaderValue> for Page {
    type Error = InvalidHeaderValue;

    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0.to_string())
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "Option<u32>")]
struct Limit(u32);

impl TryFrom<Option<u32>> for Limit {
    type Error = String;

    fn try_from(value: Option<u32>) -> Result<Self, Self::Error> {
        match value {
            Some(limit) => {
                if limit.eq(&0) {
                    Err("Limit cannot be equal zero".to_owned())
                } else {
                    Ok(Limit(limit))
                }
            }
            None => Ok(Limit(DEFAULT_PAGINATION_LIMIT)),
        }
    }
}

impl TryInto<HeaderValue> for Limit {
    type Error = InvalidHeaderValue;

    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0.to_string())
    }
}

#[derive(Debug, Deserialize)]
struct ListQueryParams {
    page: Page,
    limit: Limit,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(BIND_ADDRESS)
        .await
        .context(format!("Cannot bind to {BIND_ADDRESS}"))
        .unwrap();

    let router = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        );

    axum::serve(listener, router).await.unwrap();
}

async fn list_items(
    Query(list_params): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(PAGINATION_PAGE_HEADER, list_params.page.try_into()?);
    headers.insert(PAGINATION_LIMIT_HEADER, list_params.limit.try_into()?);

    Ok((
        StatusCode::OK,
        headers,
        Json(vec![Item {
            id: Uuid::new_v4(),
            name: ItemName("Sleeping Bag".to_owned()),
            location: ItemLocation("Calgary, AB".to_owned()),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }]),
    ))
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
            name: ItemName("Sleeping Bag".to_owned()),
            location: ItemLocation("Calgary, AB".to_owned()),
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

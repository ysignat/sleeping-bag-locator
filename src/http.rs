use axum::{
    extract::{Query, State},
    http::{header::InvalidHeaderValue, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    constants::{
        DEFAULT_PAGINATION_LIMIT,
        DEFAULT_PAGINATION_PAGE,
        MAX_LOCATION_LENGTH,
        MAX_NAME_LENGTH,
        PAGINATION_LIMIT_HEADER,
        PAGINATION_PAGE_HEADER,
    },
    dao,
    errors::AppError,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "String")]
struct Name(String);

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Empty string cannot be name of the item".to_owned())
        } else if value.len().gt(&MAX_NAME_LENGTH) {
            Err(format!("Item name cannot be longer than {MAX_NAME_LENGTH}"))
        } else {
            Ok(Name(value))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "String")]
struct Location(String);

impl TryFrom<String> for Location {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Empty string cannot be location of the item".to_owned())
        } else if value.len().gt(&MAX_LOCATION_LENGTH) {
            Err(format!(
                "Item location cannot be longer than {MAX_LOCATION_LENGTH}"
            ))
        } else {
            Ok(Location(value))
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "Option<usize>")]
struct Limit(usize);

impl TryFrom<Option<usize>> for Limit {
    type Error = String;

    fn try_from(value: Option<usize>) -> Result<Self, Self::Error> {
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
#[serde(try_from = "Option<usize>")]
struct Page(usize);

impl TryFrom<Option<usize>> for Page {
    type Error = String;

    fn try_from(value: Option<usize>) -> Result<Self, Self::Error> {
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

#[derive(Deserialize)]
struct PaginationParams {
    page: Page,
    limit: Limit,
}

#[derive(Debug, Serialize)]
struct Entity {
    id: Uuid,
    name: Name,
    location: Location,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
struct Params {
    name: Name,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct MutableParams {
    name: Name,
    location: Location,
}

pub(super) trait Crud {
    async fn list(
        pagination_params: Query<PaginationParams>,
        dao: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError>;
    async fn create(
        params: Json<Params>,
        dao: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError>;
    async fn get(
        id: Query<Uuid>,
        dao: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError>;
    async fn update(
        id: Query<Uuid>,
        mutable_params: Json<MutableParams>,
        dao: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError>;
    async fn delete(
        id: Query<Uuid>,
        dao: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError>;
}

pub(super) struct MockedCrud {}

impl Crud for MockedCrud {
    async fn list(
        Query(pagination_params): Query<PaginationParams>,
        _: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError> {
        let mut headers = HeaderMap::new();
        headers.insert(PAGINATION_PAGE_HEADER, pagination_params.page.try_into()?);
        headers.insert(PAGINATION_LIMIT_HEADER, pagination_params.limit.try_into()?);

        Ok((
            StatusCode::OK,
            headers,
            Json(vec![Entity {
                id: Uuid::new_v4(),
                name: Name("Sleeping Bag".to_owned()),
                location: Location("Calgary, AB".to_owned()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            }]),
        ))
    }

    async fn create(
        Json(params): Json<Params>,
        _: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError> {
        Ok((
            StatusCode::CREATED,
            Json(Entity {
                id: Uuid::new_v4(),
                name: params.name,
                location: params.location,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            }),
        ))
    }

    async fn get(
        Query(id): Query<Uuid>,
        _: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError> {
        Ok((
            StatusCode::OK,
            Json(Entity {
                id,
                name: Name("Sleeping Bag".to_owned()),
                location: Location("Calgary, AB".to_owned()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            }),
        ))
    }

    async fn update(
        Query(id): Query<Uuid>,
        Json(mutable_params): Json<MutableParams>,
        _: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError> {
        Ok((
            StatusCode::OK,
            Json(Entity {
                id,
                name: mutable_params.name,
                location: mutable_params.location,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            }),
        ))
    }

    async fn delete(
        _: Query<Uuid>,
        _: Option<State<impl dao::Crud>>,
    ) -> Result<impl IntoResponse, AppError> {
        Ok(StatusCode::NO_CONTENT)
    }
}

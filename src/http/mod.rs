use std::sync::Arc;

use axum::http::{header::InvalidHeaderValue, HeaderValue};
use chrono::NaiveDateTime;
use errors::AppError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dao::{self, PaginationBuilder};

mod errors;
pub mod handlers;

pub type AppState = Arc<dyn dao::DaoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Clone)]
struct Limit(usize);

impl Limit {
    const HEADER: &'static str = "Pagination-Limit";
}

impl TryInto<HeaderValue> for Limit {
    type Error = InvalidHeaderValue;

    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0.to_string())
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Page(usize);

impl Page {
    const HEADER: &'static str = "Pagination-Page";
}

impl TryInto<HeaderValue> for Page {
    type Error = InvalidHeaderValue;

    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0.to_string())
    }
}

#[derive(Deserialize, Clone)]
pub struct PaginationParams {
    page: Option<Page>,
    limit: Option<Limit>,
}

impl TryInto<dao::Pagination> for PaginationParams {
    type Error = AppError;

    fn try_into(self) -> Result<dao::Pagination, Self::Error> {
        let mut builder = PaginationBuilder::new();

        if let Some(limit) = self.limit {
            builder = builder.limit(limit.0);
        }

        if let Some(page) = self.page {
            builder = builder.page(page.0);
        }

        Ok(builder.build()?)
    }
}

#[derive(Debug, Serialize)]
struct Entity {
    id: Uuid,
    name: String,
    location: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<dao::Entity> for Entity {
    fn from(value: dao::Entity) -> Self {
        Entity {
            id: value.id(),
            name: value.name().to_owned(),
            location: value.location().to_owned(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Params {
    name: String,
    location: String,
}

impl TryInto<dao::Params> for Params {
    type Error = AppError;

    fn try_into(self) -> Result<dao::Params, Self::Error> {
        Ok(dao::ParamsBuilder::new()
            .location(self.location)
            .name(self.name)
            .build()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct MutableParams {
    name: String,
    location: String,
}

impl TryInto<dao::MutableParams> for MutableParams {
    type Error = AppError;

    fn try_into(self) -> Result<dao::MutableParams, Self::Error> {
        Ok(dao::MutableParamsBuilder::new()
            .location(self.location)
            .name(self.name)
            .build()?)
    }
}

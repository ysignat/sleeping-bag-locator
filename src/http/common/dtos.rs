use axum::http::{header::InvalidHeaderValue, HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;

use super::errors::AppError;
use crate::dao::{Pagination, PaginationBuilder};

pub const PAGINATION_LIMIT_HEADER: &str = "pagination-limit";
pub const PAGINATION_PAGE_HEADER: &str = "pagination-page";

#[derive(Deserialize, Clone)]
pub struct HttpPaginationParams {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

impl TryFrom<Pagination> for HeaderMap {
    type Error = InvalidHeaderValue;

    fn try_from(value: Pagination) -> Result<Self, Self::Error> {
        Ok(HeaderMap::from_iter(vec![
            (
                HeaderName::from_static(PAGINATION_LIMIT_HEADER),
                HeaderValue::from_str(&value.limit().to_string())?,
            ),
            (
                HeaderName::from_static(PAGINATION_PAGE_HEADER),
                HeaderValue::from_str(&value.page().to_string())?,
            ),
        ]))
    }
}

impl TryInto<Pagination> for HttpPaginationParams {
    type Error = AppError;

    fn try_into(self) -> Result<Pagination, Self::Error> {
        let mut builder = PaginationBuilder::new();

        if let Some(limit) = self.limit {
            builder = builder.limit(limit);
        }

        if let Some(page) = self.page {
            builder = builder.page(page);
        }

        Ok(builder.build()?)
    }
}

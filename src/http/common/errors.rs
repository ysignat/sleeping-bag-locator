use axum::{
    http::{header::InvalidHeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use crate::dao::PaginationBuilderError;

pub struct AppError {
    pub status_code: StatusCode,
    pub details: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, self.details).into_response()
    }
}

impl From<InvalidHeaderValue> for AppError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self {
            status_code: StatusCode::UNPROCESSABLE_ENTITY,
            details: value.to_string(),
        }
    }
}

impl From<PaginationBuilderError> for AppError {
    fn from(value: PaginationBuilderError) -> Self {
        let status_code = match value {
            PaginationBuilderError::LimitIsZero | PaginationBuilderError::PageIsZero => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

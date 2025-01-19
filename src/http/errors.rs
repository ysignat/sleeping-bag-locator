use axum::{
    http::{header::InvalidHeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use super::dao::{
    DaoCreateError,
    DaoDeleteError,
    DaoGetError,
    DaoHealthError,
    DaoListError,
    DaoUpdateError,
    PaginationBuilderError,
};
use crate::dao::{MutableParamsBuilderError, ParamsBuilderError};

pub struct AppError {
    status_code: StatusCode,
    details: String,
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

impl From<DaoCreateError> for AppError {
    fn from(value: DaoCreateError) -> Self {
        let status_code = match value {
            DaoCreateError::InvalidParams => StatusCode::UNPROCESSABLE_ENTITY,
            DaoCreateError::AlreadyExists { id: _ } => StatusCode::CONFLICT,
            DaoCreateError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<DaoDeleteError> for AppError {
    fn from(value: DaoDeleteError) -> Self {
        let status_code = match value {
            DaoDeleteError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            DaoDeleteError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<DaoGetError> for AppError {
    fn from(value: DaoGetError) -> Self {
        let status_code = match value {
            DaoGetError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            DaoGetError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<DaoHealthError> for AppError {
    fn from(value: DaoHealthError) -> Self {
        let status_code = match value {
            DaoHealthError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<DaoListError> for AppError {
    fn from(value: DaoListError) -> Self {
        let status_code = match value {
            DaoListError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<DaoUpdateError> for AppError {
    fn from(value: DaoUpdateError) -> Self {
        let status_code = match value {
            DaoUpdateError::InvalidParams => StatusCode::UNPROCESSABLE_ENTITY,
            DaoUpdateError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            DaoUpdateError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
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

impl From<ParamsBuilderError> for AppError {
    fn from(value: ParamsBuilderError) -> Self {
        let status_code = match value {
            ParamsBuilderError::LocationNotSet | ParamsBuilderError::NameNotSet => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<MutableParamsBuilderError> for AppError {
    fn from(value: MutableParamsBuilderError) -> Self {
        let status_code = match value {
            MutableParamsBuilderError::LocationNotSet | MutableParamsBuilderError::NameNotSet => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
        };
        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

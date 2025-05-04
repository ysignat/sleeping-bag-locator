use axum::http::StatusCode;

use super::{
    common::AppError,
    dao::{CreateUserError, DeleteUserError, GetUserError, UpdateUserError, UsersHealthError},
};

impl From<CreateUserError> for AppError {
    fn from(value: CreateUserError) -> Self {
        let status_code = match value {
            CreateUserError::InvalidParams => StatusCode::UNPROCESSABLE_ENTITY,
            CreateUserError::AlreadyExists { id: _ } => StatusCode::CONFLICT,
            CreateUserError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<DeleteUserError> for AppError {
    fn from(value: DeleteUserError) -> Self {
        let status_code = match value {
            DeleteUserError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            DeleteUserError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<GetUserError> for AppError {
    fn from(value: GetUserError) -> Self {
        let status_code = match value {
            GetUserError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            GetUserError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<UsersHealthError> for AppError {
    fn from(value: UsersHealthError) -> Self {
        let status_code = match value {
            UsersHealthError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<UpdateUserError> for AppError {
    fn from(value: UpdateUserError) -> Self {
        let status_code = match value {
            UpdateUserError::InvalidParams => StatusCode::UNPROCESSABLE_ENTITY,
            UpdateUserError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            UpdateUserError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

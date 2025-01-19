use axum::http::StatusCode;

use crate::{
    dao::{
        CreateItemError,
        CreateItemParamsBuilderError,
        DeleteItemError,
        GetItemError,
        ItemsHealthError,
        ListItemsError,
        UpdateItemError,
        UpdateItemParamsBuilderError,
    },
    http::common::AppError,
};

impl From<CreateItemError> for AppError {
    fn from(value: CreateItemError) -> Self {
        let status_code = match value {
            CreateItemError::InvalidParams => StatusCode::UNPROCESSABLE_ENTITY,
            CreateItemError::AlreadyExists { id: _ } => StatusCode::CONFLICT,
            CreateItemError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<DeleteItemError> for AppError {
    fn from(value: DeleteItemError) -> Self {
        let status_code = match value {
            DeleteItemError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            DeleteItemError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<GetItemError> for AppError {
    fn from(value: GetItemError) -> Self {
        let status_code = match value {
            GetItemError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            GetItemError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<ItemsHealthError> for AppError {
    fn from(value: ItemsHealthError) -> Self {
        let status_code = match value {
            ItemsHealthError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<ListItemsError> for AppError {
    fn from(value: ListItemsError) -> Self {
        let status_code = match value {
            ListItemsError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<UpdateItemError> for AppError {
    fn from(value: UpdateItemError) -> Self {
        let status_code = match value {
            UpdateItemError::InvalidParams => StatusCode::UNPROCESSABLE_ENTITY,
            UpdateItemError::NoSuchEntity { id: _ } => StatusCode::NOT_FOUND,
            UpdateItemError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<CreateItemParamsBuilderError> for AppError {
    fn from(value: CreateItemParamsBuilderError) -> Self {
        let status_code = match value {
            CreateItemParamsBuilderError::LocationNotSet
            | CreateItemParamsBuilderError::NameNotSet => StatusCode::UNPROCESSABLE_ENTITY,
        };

        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

impl From<UpdateItemParamsBuilderError> for AppError {
    fn from(value: UpdateItemParamsBuilderError) -> Self {
        let status_code = match value {
            UpdateItemParamsBuilderError::LocationNotSet
            | UpdateItemParamsBuilderError::NameNotSet => StatusCode::UNPROCESSABLE_ENTITY,
        };
        Self {
            status_code,
            details: value.to_string(),
        }
    }
}

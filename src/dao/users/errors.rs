use thiserror::Error;
use uuid::Uuid;

use super::dtos::{CreateUserValidationError, UpdateUserValidationError};

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CreateUserError {
    #[error("Cannot create entity from given params")]
    InvalidParams,
    #[error("Entity with id '{id:?}' already exists in our records")]
    AlreadyExists { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

impl From<CreateUserValidationError> for CreateUserError {
    fn from(_: CreateUserValidationError) -> Self {
        Self::InvalidParams
    }
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum GetUserError {
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum UpdateUserError {
    #[error("Cannot update entity with given params")]
    InvalidParams,
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

impl From<UpdateUserValidationError> for UpdateUserError {
    fn from(_: UpdateUserValidationError) -> Self {
        UpdateUserError::InvalidParams
    }
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DeleteUserError {
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum UsersHealthError {
    #[error("Something went wrong")]
    UnexpectedError,
}

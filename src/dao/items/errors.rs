use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ListItemsError {
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CreateItemError {
    #[error("Cannot create entity from given params")]
    InvalidParams,
    #[error("Entity with id '{id:?}' already exists in our records")]
    AlreadyExists { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum GetItemError {
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum UpdateItemError {
    #[error("Cannot update entity with given params")]
    InvalidParams,
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DeleteItemError {
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ItemsHealthError {
    #[error("Something went wrong")]
    UnexpectedError,
}

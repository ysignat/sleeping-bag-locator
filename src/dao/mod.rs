use axum::async_trait;
pub use entity::Entity;
pub use impls::{HashMapDao, MockedDao};
pub use pagination::{Pagination, PaginationBuilder, PaginationBuilderError};
pub use params::{
    MutableParams,
    MutableParamsBuilder,
    MutableParamsBuilderError,
    Params,
    ParamsBuilder,
    ParamsBuilderError,
};
use thiserror::Error;
use uuid::Uuid;

mod entity;
mod impls;
mod pagination;
mod params;

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DaoListError {
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DaoCreateError {
    #[error("Cannot create entity from given params")]
    InvalidParams,
    #[error("Entity with id '{id:?}' already exists in our records")]
    AlreadyExists { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DaoGetError {
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DaoUpdateError {
    #[error("Cannot update entity with given params")]
    InvalidParams,
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DaoDeleteError {
    #[error("Entity with id '{id:?}' doesn't exist in our records")]
    NoSuchEntity { id: Uuid },
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DaoHealthError {
    #[error("Something went wrong")]
    UnexpectedError,
}

#[async_trait]
pub trait DaoTrait {
    async fn list(&self, pagination: Pagination) -> Result<Vec<Entity>, DaoListError>;
    async fn create(&self, params: Params) -> Result<Entity, DaoCreateError>;
    async fn get(&self, id: Uuid) -> Result<Entity, DaoGetError>;
    async fn update(
        &self,
        id: Uuid,
        mutable_params: MutableParams,
    ) -> Result<Entity, DaoUpdateError>;
    async fn delete(&self, id: Uuid) -> Result<(), DaoDeleteError>;
    async fn health(&self) -> Result<(), DaoHealthError>;
}

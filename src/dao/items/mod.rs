use axum::async_trait;
pub use dtos::{
    CreateItemParams,
    CreateItemParamsBuilderError,
    CreateItemsParamsBuilder,
    Item,
    UpdateItemParams,
    UpdateItemParamsBuilder,
    UpdateItemParamsBuilderError,
};
pub use errors::{
    CreateItemError,
    DeleteItemError,
    GetItemError,
    ItemsHealthError,
    ListItemsError,
    UpdateItemError,
};
pub use impls::{ItemsHashMapDao, ItemsMockedDao};
use uuid::Uuid;

use crate::dao::common::Pagination;

mod dtos;
mod errors;
mod impls;

#[async_trait]
pub trait ItemsDao {
    async fn list(&self, pagination: Pagination) -> Result<Vec<Item>, ListItemsError>;
    async fn create(&self, params: CreateItemParams) -> Result<Item, CreateItemError>;
    async fn get(&self, id: Uuid) -> Result<Item, GetItemError>;
    async fn update(&self, id: Uuid, params: UpdateItemParams) -> Result<Item, UpdateItemError>;
    async fn delete(&self, id: Uuid) -> Result<(), DeleteItemError>;
    async fn health(&self) -> Result<(), ItemsHealthError>;
}

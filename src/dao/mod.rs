use anyhow::Result;
use axum::async_trait;
pub use entity::Entity;
pub use impls::{HashMapDao, MockedDao};
pub use pagination::{Pagination, PaginationBuilder};
pub use params::{MutableParams, MutableParamsBuilder, Params, ParamsBuilder};
use uuid::Uuid;

mod entity;
mod impls;
mod pagination;
mod params;

#[async_trait]
pub trait DaoTrait {
    async fn list(&self, pagination: Pagination) -> Result<Vec<Entity>>;
    async fn create(&self, params: Params) -> Result<Entity>;
    async fn get(&self, id: Uuid) -> Result<Entity>;
    async fn update(&self, id: Uuid, mutable_params: MutableParams) -> Result<Entity>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn health(&self) -> Result<()>;
}

use axum::async_trait;
use uuid::Uuid;

use crate::dao::{
    entity::{Entity, EntityBuilder},
    pagination::Pagination,
    params::{MutableParams, Params},
    DaoTrait,
};

pub struct MockedDao {}

#[async_trait]
impl DaoTrait for MockedDao {
    async fn list(&self, _: Pagination) -> anyhow::Result<Vec<Entity>> {
        let entity = EntityBuilder::new()
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()?;

        Ok(vec![entity])
    }

    async fn create(&self, params: Params) -> anyhow::Result<Entity> {
        let entity = params.try_into()?;

        Ok(entity)
    }

    async fn get(&self, id: Uuid) -> anyhow::Result<Entity> {
        let entity = EntityBuilder::new()
            .id(id)
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()?;

        Ok(entity)
    }

    async fn update(&self, id: Uuid, mutable_params: MutableParams) -> anyhow::Result<Entity> {
        let entity = EntityBuilder::new()
            .id(id)
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()?
            .try_mutate(&mutable_params)?;

        Ok(entity)
    }

    async fn delete(&self, _: Uuid) -> anyhow::Result<()> {
        Ok(())
    }
}

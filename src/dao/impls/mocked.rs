use axum::async_trait;
use uuid::Uuid;

use crate::dao::{
    entity::{Entity, EntityBuilder},
    pagination::Pagination,
    params::{MutableParams, Params},
    DaoCreateError,
    DaoDeleteError,
    DaoGetError,
    DaoHealthError,
    DaoListError,
    DaoTrait,
    DaoUpdateError,
};

pub struct MockedDao {}

#[async_trait]
impl DaoTrait for MockedDao {
    async fn list(&self, _: Pagination) -> Result<Vec<Entity>, DaoListError> {
        let entity = EntityBuilder::new()
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()
            .or(Err(DaoListError::UnexpectedError))?;

        Ok(vec![entity])
    }

    async fn create(&self, params: Params) -> Result<Entity, DaoCreateError> {
        let entity = params.try_into().or(Err(DaoCreateError::InvalidParams))?;

        Ok(entity)
    }

    async fn get(&self, id: Uuid) -> Result<Entity, DaoGetError> {
        let entity = EntityBuilder::new()
            .id(id)
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()
            .or(Err(DaoGetError::UnexpectedError))?;

        Ok(entity)
    }

    async fn update(
        &self,
        id: Uuid,
        mutable_params: MutableParams,
    ) -> Result<Entity, DaoUpdateError> {
        let entity = EntityBuilder::new()
            .id(id)
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()
            .or(Err(DaoUpdateError::UnexpectedError))?
            .try_mutate(&mutable_params)
            .or(Err(DaoUpdateError::InvalidParams))?;

        Ok(entity)
    }

    async fn delete(&self, _: Uuid) -> Result<(), DaoDeleteError> {
        Ok(())
    }

    async fn health(&self) -> Result<(), DaoHealthError> {
        Ok(())
    }
}

use axum::async_trait;
use uuid::Uuid;

use crate::dao::{
    common::Pagination,
    items::{
        dtos::ItemBuilder,
        CreateItemError,
        CreateItemParams,
        DeleteItemError,
        GetItemError,
        Item,
        ItemsDao,
        ItemsHealthError,
        ListItemsError,
        UpdateItemError,
        UpdateItemParams,
    },
};

pub struct ItemsMockedDao {}

#[async_trait]
impl ItemsDao for ItemsMockedDao {
    async fn list(&self, _: Pagination) -> Result<Vec<Item>, ListItemsError> {
        let entity = ItemBuilder::new()
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()
            .or(Err(ListItemsError::UnexpectedError))?;

        Ok(vec![entity])
    }

    async fn create(&self, params: CreateItemParams) -> Result<Item, CreateItemError> {
        let entity = params.try_into().or(Err(CreateItemError::InvalidParams))?;

        Ok(entity)
    }

    async fn get(&self, id: Uuid) -> Result<Item, GetItemError> {
        let entity = ItemBuilder::new()
            .id(id)
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()
            .or(Err(GetItemError::UnexpectedError))?;

        Ok(entity)
    }

    async fn update(&self, id: Uuid, params: UpdateItemParams) -> Result<Item, UpdateItemError> {
        let entity = ItemBuilder::new()
            .id(id)
            .name("Sleeping Bag".to_owned())
            .location("Calgary, AB".to_owned())
            .build()
            .or(Err(UpdateItemError::UnexpectedError))?
            .try_update(&params)
            .or(Err(UpdateItemError::InvalidParams))?;

        Ok(entity)
    }

    async fn delete(&self, _: Uuid) -> Result<(), DeleteItemError> {
        Ok(())
    }

    async fn health(&self) -> Result<(), ItemsHealthError> {
        Ok(())
    }
}

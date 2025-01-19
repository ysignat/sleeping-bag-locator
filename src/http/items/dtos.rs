use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dao::{
    CreateItemParams,
    CreateItemParamsBuilderError,
    CreateItemsParamsBuilder,
    Item,
    UpdateItemParams,
    UpdateItemParamsBuilder,
    UpdateItemParamsBuilderError,
};

#[derive(Debug, Serialize)]
pub struct HttpItem {
    id: Uuid,
    name: String,
    location: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Item> for HttpItem {
    fn from(value: Item) -> Self {
        HttpItem {
            id: value.id(),
            name: value.name().to_owned(),
            location: value.location().to_owned(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpCreateItemParams {
    name: String,
    location: String,
}

impl TryInto<CreateItemParams> for HttpCreateItemParams {
    type Error = CreateItemParamsBuilderError;

    fn try_into(self) -> Result<CreateItemParams, Self::Error> {
        CreateItemsParamsBuilder::new()
            .location(self.location)
            .name(self.name)
            .build()
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpUpdateItemParams {
    name: String,
    location: String,
}

impl TryInto<UpdateItemParams> for HttpUpdateItemParams {
    type Error = UpdateItemParamsBuilderError;

    fn try_into(self) -> Result<UpdateItemParams, Self::Error> {
        UpdateItemParamsBuilder::new()
            .location(self.location)
            .name(self.name)
            .build()
    }
}

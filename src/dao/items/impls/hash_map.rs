use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use axum::async_trait;
use uuid::Uuid;

use crate::dao::{
    common::Pagination,
    items::{
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

#[derive(Clone)]
pub struct ItemsHashMapDao(Arc<RwLock<HashMap<Uuid, Item>>>);

impl ItemsHashMapDao {
    pub fn new() -> Self {
        ItemsHashMapDao(Arc::new(RwLock::new(HashMap::new())))
    }

    fn read(&self) -> RwLockReadGuard<HashMap<Uuid, Item>> {
        self.0.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<HashMap<Uuid, Item>> {
        self.0.write().unwrap()
    }
}

#[async_trait]
impl ItemsDao for ItemsHashMapDao {
    async fn list(&self, pagination: Pagination) -> Result<Vec<Item>, ListItemsError> {
        let data = self.read();
        let mut vec: Vec<&Item> = data.values().collect();

        vec.sort_by_key(|x| x.updated_at());

        Ok(vec
            .into_iter()
            .skip((pagination.page() - 1) * pagination.limit())
            .take(pagination.limit())
            .map(ToOwned::to_owned)
            .collect())
    }

    async fn create(&self, params: CreateItemParams) -> Result<Item, CreateItemError> {
        let mut data = self.write();
        let entity: Item = params.try_into().or(Err(CreateItemError::InvalidParams))?;

        if let Entry::Vacant(e) = data.entry(entity.id()) {
            Ok(e.insert(entity).to_owned())
        } else {
            Err(CreateItemError::AlreadyExists { id: entity.id() }) // Could only happen on a UUID collision
        }
    }

    async fn get(&self, id: Uuid) -> Result<Item, GetItemError> {
        let data = self.read();
        Ok(data
            .get(&id)
            .cloned()
            .ok_or(GetItemError::NoSuchEntity { id })?)
    }

    async fn update(&self, id: Uuid, params: UpdateItemParams) -> Result<Item, UpdateItemError> {
        let mut data = self.write();
        if let Some(entity) = data.get(&id) {
            let updated = entity
                .clone()
                .try_update(&params)
                .or(Err(UpdateItemError::InvalidParams))?;

            let _ = data.insert(id, updated.clone());
            Ok(updated)
        } else {
            Err(UpdateItemError::NoSuchEntity { id })
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), DeleteItemError> {
        let mut data = self.write();
        data.remove(&id)
            .ok_or(DeleteItemError::NoSuchEntity { id })
            .and(Ok(()))
    }

    async fn health(&self) -> Result<(), ItemsHealthError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use super::*;

    #[tokio::test]
    async fn create() {
        let dao = ItemsHashMapDao::new();
        let params: CreateItemParams = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");

        assert_eq!(params.location(), entity.location());
        assert_eq!(params.name(), entity.name());
        assert_eq!(entity.created_at(), entity.updated_at());
    }

    #[tokio::test]
    async fn get() {
        let dao = ItemsHashMapDao::new();
        let params: CreateItemParams = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        let result = dao.get(entity.id()).await.unwrap();
        println!("{result:#?}");

        assert_eq!(entity, result);
    }

    #[tokio::test]
    async fn get_non_existent() {
        let dao = ItemsHashMapDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let result = dao.get(id).await;
        println!("{result:#?}");

        assert_eq!(result, Err(GetItemError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn delete() {
        let dao = ItemsHashMapDao::new();
        let params: CreateItemParams = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        dao.delete(entity.id()).await.unwrap();
    }

    #[tokio::test]
    async fn delete_non_existent() {
        let dao = ItemsHashMapDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let result = dao.delete(id).await;
        println!("{result:#?}");

        assert_eq!(result, Err(DeleteItemError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn update() {
        let dao = ItemsHashMapDao::new();
        let create_params: CreateItemParams = Faker.fake();
        println!("{create_params:#?}");
        let entity = dao.create(create_params.clone()).await.unwrap();
        println!("{entity:#?}");
        let update_params: UpdateItemParams = Faker.fake();
        println!("{update_params:#?}");

        let update_result = dao
            .update(entity.id(), update_params.clone())
            .await
            .unwrap();
        println!("{update_result:#?}");

        assert_eq!(update_result.name(), update_params.name());
        assert_eq!(update_result.location(), update_params.location());
        assert_eq!(update_result.created_at(), entity.created_at());
        assert!(update_result.updated_at().gt(&entity.updated_at()));

        let get_result = dao.get(entity.id()).await.unwrap();
        println!("{get_result:#?}");

        assert_eq!(get_result, update_result);
    }

    #[tokio::test]
    async fn update_non_existent() {
        let dao = ItemsHashMapDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let params = Faker.fake();
        println!("{params:#?}");
        let result = dao.update(id, params).await;
        println!("{result:#?}");

        assert_eq!(result, Err(UpdateItemError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn list_empty() {
        let dao = ItemsHashMapDao::new();
        let pagination = Faker.fake();
        println!("{pagination:#?}");

        let result = dao.list(pagination).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn list() {
        let dao = ItemsHashMapDao::new();
        let pagination: Pagination = Faker.fake();
        println!("{pagination:#?}");

        let count = 2 * pagination.page() * pagination.limit();
        let mut vec = Vec::new();

        for i in 0..count {
            let params = Faker.fake();
            let entity = dao.create(params).await.unwrap();
            println!("{entity:#?}");
            if i >= (pagination.page() - 1) * pagination.limit()
                && i < pagination.page() * pagination.limit()
            {
                vec.push(entity);
            }
        }
        println!("{vec:#?}");

        let result = dao.list(pagination).await.unwrap();
        assert_eq!(result, vec);
    }
}

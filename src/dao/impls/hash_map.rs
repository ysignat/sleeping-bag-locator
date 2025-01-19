use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use axum::async_trait;
use uuid::Uuid;

use crate::dao::{
    entity::Entity,
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

#[derive(Clone)]
pub struct HashMapDao(Arc<RwLock<HashMap<Uuid, Entity>>>);

impl HashMapDao {
    pub fn new() -> Self {
        HashMapDao(Arc::new(RwLock::new(HashMap::new())))
    }

    fn read(&self) -> RwLockReadGuard<HashMap<Uuid, Entity>> {
        self.0.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<HashMap<Uuid, Entity>> {
        self.0.write().unwrap()
    }
}

#[async_trait]
impl DaoTrait for HashMapDao {
    async fn list(&self, pagination: Pagination) -> Result<Vec<Entity>, DaoListError> {
        let data = self.read();
        let mut vec: Vec<&Entity> = data.values().collect();

        vec.sort_by_key(|x| x.updated_at());

        Ok(vec
            .into_iter()
            .skip((pagination.page() - 1) * pagination.limit())
            .take(pagination.limit())
            .map(ToOwned::to_owned)
            .collect())
    }

    async fn create(&self, params: Params) -> Result<Entity, DaoCreateError> {
        let mut data = self.write();
        let entity: Entity = params.try_into().or(Err(DaoCreateError::InvalidParams))?;

        if let Entry::Vacant(e) = data.entry(entity.id()) {
            Ok(e.insert(entity).to_owned())
        } else {
            Err(DaoCreateError::AlreadyExists { id: entity.id() }) // Could only happen on a UUID collision
        }
    }

    async fn get(&self, id: Uuid) -> Result<Entity, DaoGetError> {
        let data = self.read();
        Ok(data
            .get(&id)
            .cloned()
            .ok_or(DaoGetError::NoSuchEntity { id })?)
    }

    async fn update(
        &self,
        id: Uuid,
        mutable_params: MutableParams,
    ) -> Result<Entity, DaoUpdateError> {
        let mut data = self.write();
        if let Some(entity) = data.get(&id) {
            let updated = entity
                .clone()
                .try_mutate(&mutable_params)
                .or(Err(DaoUpdateError::InvalidParams))?;

            let _ = data.insert(id, updated.clone());
            Ok(updated)
        } else {
            Err(DaoUpdateError::NoSuchEntity { id })
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), DaoDeleteError> {
        let mut data = self.write();
        data.remove(&id)
            .ok_or(DaoDeleteError::NoSuchEntity { id })
            .and(Ok(()))
    }

    async fn health(&self) -> Result<(), DaoHealthError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use super::*;

    #[tokio::test]
    async fn create() {
        let dao = HashMapDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");

        assert_eq!(params.location(), entity.location());
        assert_eq!(params.name(), entity.name());
        assert_eq!(entity.created_at(), entity.updated_at());
    }

    #[tokio::test]
    async fn get() {
        let dao = HashMapDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        let result = dao.get(entity.id()).await.unwrap();
        println!("{result:#?}");

        assert_eq!(entity, result);
    }

    #[tokio::test]
    async fn get_non_existent() {
        let dao = HashMapDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let result = dao.get(id).await;
        println!("{result:#?}");

        assert_eq!(result, Err(DaoGetError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn delete() {
        let dao = HashMapDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        dao.delete(entity.id()).await.unwrap();
    }

    #[tokio::test]
    async fn delete_non_existent() {
        let dao = HashMapDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let result = dao.delete(id).await;
        println!("{result:#?}");

        assert_eq!(result, Err(DaoDeleteError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn update() {
        let dao = HashMapDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        let mutable_params: MutableParams = Faker.fake();
        println!("{mutable_params:#?}");

        let update_result = dao
            .update(entity.id(), mutable_params.clone())
            .await
            .unwrap();
        println!("{update_result:#?}");

        assert_eq!(update_result.name(), mutable_params.name());
        assert_eq!(update_result.location(), mutable_params.location());
        assert_eq!(update_result.created_at(), entity.created_at());
        assert!(update_result.updated_at().gt(&entity.updated_at()));

        let get_result = dao.get(entity.id()).await.unwrap();
        println!("{get_result:#?}");

        assert_eq!(get_result, update_result);
    }

    #[tokio::test]
    async fn update_non_existent() {
        let dao = HashMapDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let mutable_params = Faker.fake();
        println!("{mutable_params:#?}");
        let result = dao.update(id, mutable_params).await;
        println!("{result:#?}");

        assert_eq!(result, Err(DaoUpdateError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn list_empty() {
        let dao = HashMapDao::new();
        let pagination = Faker.fake();
        println!("{pagination:#?}");

        let result = dao.list(pagination).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn list() {
        let dao = HashMapDao::new();
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

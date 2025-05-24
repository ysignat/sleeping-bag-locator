use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use axum::async_trait;
use uuid::Uuid;

use super::{
    dtos::{CreateUserParams, UpdateUserParams, User},
    errors::{CreateUserError, DeleteUserError, GetUserError, UpdateUserError, UsersHealthError},
    interface::UsersDao,
};

#[derive(Clone)]
pub struct UsersHashMapDao(Arc<RwLock<HashMap<Uuid, User>>>);

impl UsersHashMapDao {
    pub fn new() -> Self {
        UsersHashMapDao(Arc::new(RwLock::new(HashMap::new())))
    }

    fn read(&self) -> RwLockReadGuard<HashMap<Uuid, User>> {
        self.0.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<HashMap<Uuid, User>> {
        self.0.write().unwrap()
    }
}

#[async_trait]
impl UsersDao for UsersHashMapDao {
    async fn create(&self, params: CreateUserParams) -> Result<User, CreateUserError> {
        let mut data = self.write();
        let entity: User = params.try_into()?;

        if let Entry::Vacant(e) = data.entry(entity.id()) {
            Ok(e.insert(entity).to_owned())
        } else {
            Err(CreateUserError::AlreadyExists { id: entity.id() }) // Could only happen on a UUID collision
        }
    }

    async fn get(&self, id: Uuid) -> Result<User, GetUserError> {
        let data = self.read();
        Ok(data
            .get(&id)
            .cloned()
            .ok_or(GetUserError::NoSuchEntity { id })?)
    }

    async fn update(&self, id: Uuid, params: UpdateUserParams) -> Result<User, UpdateUserError> {
        let mut data = self.write();
        if let Some(entity) = data.get_mut(&id) {
            entity.try_update(params)?;

            Ok(entity.to_owned())
        } else {
            Err(UpdateUserError::NoSuchEntity { id })
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), DeleteUserError> {
        let mut data = self.write();
        data.remove(&id)
            .ok_or(DeleteUserError::NoSuchEntity { id })
            .and(Ok(()))
    }

    async fn health(&self) -> Result<(), UsersHealthError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use super::*;

    #[tokio::test]
    async fn create() {
        let dao = UsersHashMapDao::new();
        let params: CreateUserParams = Faker.fake();
        println!("{params:#?}");

        let err = dao
            .create(CreateUserParams::new(
                String::new(),
                params.auth_type(),
                params.external_id().to_owned(),
            ))
            .await;

        assert_eq!(err, Err(CreateUserError::InvalidParams));

        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");

        assert_eq!(params.name(), entity.name());
        assert_eq!(params.auth_type(), entity.auth_type());
        assert_eq!(params.external_id(), entity.external_id());
        assert_eq!(entity.created_at(), entity.updated_at());
    }

    #[tokio::test]
    async fn get() {
        let dao = UsersHashMapDao::new();
        let params: CreateUserParams = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        let result = dao.get(entity.id()).await.unwrap();
        println!("{result:#?}");

        assert_eq!(entity, result);

        let id = Faker.fake();
        println!("{id:#?}");
        let err = dao.get(id).await;
        println!("{err:#?}");

        assert_eq!(err, Err(GetUserError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn delete() {
        let dao = UsersHashMapDao::new();
        let params: CreateUserParams = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        dao.delete(entity.id()).await.unwrap();

        let id = Faker.fake();

        let err = dao.delete(id).await;
        println!("{err:#?}");

        assert_eq!(err, Err(DeleteUserError::NoSuchEntity { id }));
    }

    #[tokio::test]
    async fn update() {
        let dao = UsersHashMapDao::new();
        let create_params: CreateUserParams = Faker.fake();
        println!("{create_params:#?}");
        let entity = dao.create(create_params.clone()).await.unwrap();
        println!("{entity:#?}");
        let update_params: UpdateUserParams = Faker.fake();
        println!("{update_params:#?}");

        let update_result = dao
            .update(entity.id(), update_params.clone())
            .await
            .unwrap();
        println!("{update_result:#?}");

        assert_eq!(update_result.name(), update_params.name());
        assert_eq!(update_result.created_at(), entity.created_at());
        assert!(update_result.updated_at().gt(&entity.updated_at()));

        let get_result = dao.get(entity.id()).await.unwrap();
        println!("{get_result:#?}");

        assert_eq!(get_result, update_result);

        let id = Faker.fake();
        println!("{id:#?}");
        let err = dao.update(id, update_params).await;
        println!("{err:#?}");

        assert_eq!(err, Err(UpdateUserError::NoSuchEntity { id }));

        let err = dao
            .update(entity.id(), UpdateUserParams::new(String::new()))
            .await;

        assert_eq!(err, Err(UpdateUserError::InvalidParams));
    }
}

use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDateTime, Utc};
#[cfg(test)]
use fake::{faker::address::en::CityName, faker::lorem::en::Word, Dummy};
use uuid::Uuid;

use crate::constants::{DEFAULT_PAGINATION_LIMIT, DEFAULT_PAGINATION_PAGE};

#[cfg_attr(test, derive(Debug, Dummy))]
pub(super) struct PaginationParams {
    #[cfg_attr(test, dummy(faker = "1..10"))]
    page: usize,
    #[cfg_attr(test, dummy(faker = "1..10"))]
    limit: usize,
}

impl PaginationParams {
    pub(super) fn new() -> Self {
        Self {
            page: DEFAULT_PAGINATION_PAGE,
            limit: DEFAULT_PAGINATION_LIMIT,
        }
    }
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
struct Entity {
    id: Uuid,
    name: String,
    location: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[cfg_attr(test, derive(Dummy, Clone, Debug))]
pub(super) struct Params {
    #[cfg_attr(test, dummy(faker = "Word()"))]
    name: String,
    #[cfg_attr(test, dummy(faker = "CityName()"))]
    location: String,
}

#[cfg_attr(test, derive(Dummy, Clone, Debug))]
pub(super) struct MutableParams {
    #[cfg_attr(test, dummy(faker = "Word()"))]
    name: String,
    #[cfg_attr(test, dummy(faker = "CityName()"))]
    location: String,
}

pub(super) trait Crud {
    async fn list(&self, pagination_params: PaginationParams) -> Result<Vec<Entity>>;
    async fn create(&self, params: Params) -> Result<Entity>;
    async fn get(&self, id: Uuid) -> Result<Entity>;
    async fn update(&self, id: Uuid, mutable_params: MutableParams) -> Result<Entity>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

struct MockedDao(Arc<RwLock<HashMap<Uuid, Entity>>>);

impl MockedDao {
    fn new() -> Self {
        MockedDao(Arc::new(RwLock::new(HashMap::new())))
    }

    fn read(&self) -> RwLockReadGuard<HashMap<Uuid, Entity>> {
        self.0.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<HashMap<Uuid, Entity>> {
        self.0.write().unwrap()
    }
}

impl Crud for MockedDao {
    async fn list(&self, pagination_params: PaginationParams) -> Result<Vec<Entity>> {
        let data = self.read();
        let mut vec: Vec<&Entity> = data.values().collect();

        vec.sort_by_key(|x| x.updated_at);

        Ok(vec
            .into_iter()
            .skip((pagination_params.page - 1) * pagination_params.limit)
            .take(pagination_params.limit)
            .map(ToOwned::to_owned)
            .collect())
    }

    async fn create(&self, params: Params) -> Result<Entity> {
        let mut data = self.write();
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4();

        if let Entry::Vacant(e) = data.entry(id) {
            Ok(e.insert(Entity {
                id,
                name: params.name,
                location: params.location,
                created_at: now,
                updated_at: now,
            })
            .to_owned())
        } else {
            Err(anyhow!("Already Exists")) // Could only happen on a UUID collision
        }
    }

    async fn get(&self, id: Uuid) -> Result<Entity> {
        let data = self.read();
        data.get(&id).cloned().context("No Such Entity")
    }

    async fn update(&self, id: Uuid, mutable_params: MutableParams) -> Result<Entity> {
        let mut data = self.write();
        let now = Utc::now().naive_utc();
        if let Some(entity) = data.get(&id) {
            let updated = Entity {
                id,
                name: mutable_params.name,
                location: mutable_params.location,
                created_at: entity.created_at,
                updated_at: now,
            };
            let _ = data.insert(id, updated.clone());
            Ok(updated)
        } else {
            Err(anyhow!("No Such Entity"))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut data = self.write();
        data.remove(&id).context("No Such Entity").and(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use super::*;

    #[tokio::test]
    async fn create() {
        let dao = MockedDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");

        assert_eq!(params.location, entity.location);
        assert_eq!(params.name, entity.name);
        assert_eq!(entity.created_at, entity.updated_at);
    }

    #[tokio::test]
    async fn get() {
        let dao = MockedDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        let result = dao.get(entity.id).await.unwrap();
        println!("{result:#?}");

        assert_eq!(entity, result);
    }

    #[tokio::test]
    async fn get_non_existent() {
        let dao = MockedDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let result = dao.get(id).await;
        println!("{result:#?}");

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn delete() {
        let dao = MockedDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        dao.delete(entity.id).await.unwrap();
    }

    #[tokio::test]
    async fn delete_non_existent() {
        let dao = MockedDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let result = dao.delete(id).await;
        println!("{result:#?}");

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn update() {
        let dao = MockedDao::new();
        let params: Params = Faker.fake();
        println!("{params:#?}");
        let entity = dao.create(params.clone()).await.unwrap();
        println!("{entity:#?}");
        let mutable_params: MutableParams = Faker.fake();
        println!("{mutable_params:#?}");

        let update_result = dao.update(entity.id, mutable_params.clone()).await.unwrap();
        println!("{update_result:#?}");

        assert_eq!(update_result.name, mutable_params.name);
        assert_eq!(update_result.location, mutable_params.location);
        assert_eq!(update_result.created_at, entity.created_at);
        assert!(update_result.updated_at.gt(&entity.updated_at));

        let get_result = dao.get(entity.id).await.unwrap();
        println!("{get_result:#?}");

        assert_eq!(get_result, update_result);
    }

    #[tokio::test]
    async fn update_non_existent() {
        let dao = MockedDao::new();
        let id = Faker.fake();
        println!("{id:#?}");
        let mutable_params = Faker.fake();
        println!("{mutable_params:#?}");
        let result = dao.update(id, mutable_params).await;
        println!("{result:#?}");

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn list_empty() {
        let dao = MockedDao::new();
        let pagination_params = Faker.fake();
        println!("{pagination_params:#?}");

        let result = dao.list(pagination_params).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn list() {
        let dao = MockedDao::new();
        let pagination_params: PaginationParams = Faker.fake();
        println!("{pagination_params:#?}");

        let count = 2 * pagination_params.page * pagination_params.limit;
        let mut vec = Vec::new();

        for i in 0..count {
            let params = Faker.fake();
            let entity = dao.create(params).await.unwrap();
            println!("{entity:#?}");
            if i >= (pagination_params.page - 1) * pagination_params.limit
                && i < pagination_params.page * pagination_params.limit
            {
                vec.push(entity);
            }
        }
        println!("{vec:#?}");

        let result = dao.list(pagination_params).await.unwrap();
        assert_eq!(result, vec);
    }
}

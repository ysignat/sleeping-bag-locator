use std::collections::{hash_map::Entry, HashMap};

use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{DEFAULT_PAGINATION_LIMIT, DEFAULT_PAGINATION_PAGE};

#[derive(Debug, Deserialize)]
pub(super) struct PaginationParams {
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SearchPredicates {
    name: Option<String>,
    location: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub(super) struct Entity {
    id: Uuid,
    name: String,
    location: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub(super) struct Params {
    name: String,
    location: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct MutableParams {
    name: String,
    location: String,
}

trait Crud {
    async fn list_items(
        self,
        pagination_params: PaginationParams,
        search_predicates: SearchPredicates,
    ) -> Result<Vec<Entity>>;
    async fn create_items(&mut self, params: Params) -> Result<Entity>;
    async fn get_item(self, id: Uuid) -> Result<Entity>;
    async fn update_item(&mut self, id: Uuid, mutable_params: MutableParams) -> Result<Entity>;
    async fn delete_item(&mut self, id: Uuid) -> Result<()>;
}

impl Crud for HashMap<Uuid, Entity> {
    async fn list_items(
        self,
        pagination_params: PaginationParams,
        search_predicates: SearchPredicates,
    ) -> Result<Vec<Entity>> {
        // let mut vec: Vec<Entity> = self.into_values().collect();
        let mut vec: Vec<&Entity> = self
            .iter()
            .filter(|(_, x)| {
                if let Some(predicate) = search_predicates.name.clone() {
                    x.name.starts_with(&predicate)
                } else {
                    true
                }
            })
            .filter(|(_, x)| {
                if let Some(predicate) = search_predicates.location.clone() {
                    x.location.starts_with(&predicate)
                } else {
                    true
                }
            })
            .map(|(_, x)| x)
            .collect();

        vec.sort_by_key(|x| x.updated_at);

        Ok(vec
            .into_iter()
            .skip(
                (pagination_params.page.unwrap_or(DEFAULT_PAGINATION_PAGE) - 1)
                    * pagination_params.limit.unwrap_or(DEFAULT_PAGINATION_LIMIT),
            )
            .map(ToOwned::to_owned)
            .collect())
    }

    async fn create_items(&mut self, params: Params) -> Result<Entity> {
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4();

        if let Entry::Vacant(e) = self.entry(id) {
            Ok({
                e.insert(Entity {
                    id,
                    name: params.name,
                    location: params.location,
                    created_at: now,
                    updated_at: now,
                });
                None
            }
            .unwrap())
        } else {
            Err(anyhow!("Already Exists"))
        }
    }

    async fn get_item(self, id: Uuid) -> Result<Entity> {
        self.get(&id).cloned().context("No Such Entity")
    }

    async fn update_item(&mut self, id: Uuid, mutable_params: MutableParams) -> Result<Entity> {
        let now = Utc::now().naive_utc();
        if let Some(entity) = self.get(&id) {
            let updated = Entity {
                id,
                name: mutable_params.name,
                location: mutable_params.location,
                created_at: entity.created_at,
                updated_at: now,
            };
            let _ = self.insert(id, updated.clone());
            Ok(updated)
        } else {
            Err(anyhow!("No Such Entity"))
        }
    }

    async fn delete_item(&mut self, id: Uuid) -> Result<()> {
        self.remove(&id).context("No Such Entity").and(Ok(()))
    }
}

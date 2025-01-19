use std::sync::Arc;

pub use items::{create_item, delete_item, get_item, health, list_items, update_item};

use crate::dao::ItemsDao;

mod common;
mod items;

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<dyn ItemsDao + Send + Sync>,
}

use axum::async_trait;
use uuid::Uuid;

use super::{
    dtos::{CreateUserParams, UpdateUserParams, User},
    errors::{CreateUserError, DeleteUserError, GetUserError, UpdateUserError, UsersHealthError},
};

#[async_trait]
pub trait UsersDao {
    async fn create(&self, params: CreateUserParams) -> Result<User, CreateUserError>;
    async fn get(&self, id: Uuid) -> Result<User, GetUserError>;
    async fn update(&self, id: Uuid, params: UpdateUserParams) -> Result<User, UpdateUserError>;
    async fn delete(&self, id: Uuid) -> Result<(), DeleteUserError>;
    async fn health(&self) -> Result<(), UsersHealthError>;
}

use axum::async_trait;
use uuid::Uuid;

use super::{
    dtos::{CreateUserParams, UpdateUserParams, User, UserAuthType},
    errors::{CreateUserError, DeleteUserError, GetUserError, UpdateUserError, UsersHealthError},
    interface::UsersDao,
};

pub struct UsersMockedDao {}

#[async_trait]
impl UsersDao for UsersMockedDao {
    async fn create(&self, params: CreateUserParams) -> Result<User, CreateUserError> {
        if params.external_id().eq("AlreadyExistingID") {
            return Err(CreateUserError::AlreadyExists { id: Uuid::new_v4() });
        }

        Ok(params.try_into()?)
    }

    async fn get(&self, id: Uuid) -> Result<User, GetUserError> {
        if id.is_nil() {
            return Err(GetUserError::NoSuchEntity { id });
        }

        CreateUserParams::new(
            "Sleeping Bag".to_owned(),
            UserAuthType::Github,
            "awesome-github-id".to_owned(),
        )
        .try_into()
        .or(Err(GetUserError::UnexpectedError))
    }

    async fn update(&self, id: Uuid, params: UpdateUserParams) -> Result<User, UpdateUserError> {
        if id.is_nil() {
            return Err(UpdateUserError::NoSuchEntity { id });
        }

        let mut user: User = CreateUserParams::new(
            "Sleeping Bag".to_owned(),
            UserAuthType::Github,
            "awesome-github-id".to_owned(),
        )
        .try_into()
        .or(Err(UpdateUserError::UnexpectedError))?;

        user.try_update(params)?;

        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DeleteUserError> {
        if id.is_nil() {
            return Err(DeleteUserError::NoSuchEntity { id });
        }

        Ok(())
    }

    async fn health(&self) -> Result<(), UsersHealthError> {
        Ok(())
    }
}

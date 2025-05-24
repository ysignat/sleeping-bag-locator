use chrono::NaiveDateTime;
#[cfg(test)]
use fake::{Dummy, Faker, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::dao::{User, UserAuthType};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum HttpUserAuthType {
    Github,
}

#[cfg(test)]
impl Dummy<Faker> for HttpUserAuthType {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Self::Github
    }
}

impl From<UserAuthType> for HttpUserAuthType {
    fn from(value: UserAuthType) -> Self {
        match value {
            UserAuthType::Github => HttpUserAuthType::Github,
        }
    }
}

impl From<HttpUserAuthType> for UserAuthType {
    fn from(value: HttpUserAuthType) -> Self {
        match value {
            HttpUserAuthType::Github => UserAuthType::Github,
        }
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, PartialEq, Eq))]
pub struct HttpUser {
    id: Uuid,
    name: String,
    auth_type: HttpUserAuthType,
    external_id: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl HttpUser {
    pub fn new(
        id: Uuid,
        name: String,
        auth_type: HttpUserAuthType,
        external_id: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            name,
            auth_type,
            external_id,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn auth_type(&self) -> HttpUserAuthType {
        self.auth_type
    }

    pub fn external_id(&self) -> &str {
        &self.external_id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

impl From<User> for HttpUser {
    fn from(value: User) -> Self {
        HttpUser {
            id: value.id(),
            name: value.name().to_owned(),
            auth_type: value.auth_type().into(),
            external_id: value.external_id().to_owned(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

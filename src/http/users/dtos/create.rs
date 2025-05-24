#[cfg(test)]
use fake::{faker::lorem::en::Word, faker::name::en::Name, Dummy};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;

use super::{dao::CreateUserParams, entity::HttpUserAuthType};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Dummy, Clone, PartialEq, Eq, Serialize))]
pub struct HttpCreateUserParams {
    #[cfg_attr(test, dummy(faker = "Name()"))]
    name: String,
    auth_type: HttpUserAuthType,
    #[cfg_attr(test, dummy(faker = "Word()"))]
    external_id: String,
}

impl HttpCreateUserParams {
    pub fn new(name: String, auth_type: HttpUserAuthType, external_id: String) -> Self {
        Self {
            name,
            auth_type,
            external_id,
        }
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
}

impl From<HttpCreateUserParams> for CreateUserParams {
    fn from(val: HttpCreateUserParams) -> Self {
        CreateUserParams::new(val.name, val.auth_type.into(), val.external_id)
    }
}

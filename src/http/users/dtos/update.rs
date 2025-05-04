#[cfg(test)]
use fake::{faker::name::en::Name, Dummy};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;

use super::dao::UpdateUserParams;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Dummy, Serialize))]
pub struct HttpUpdateUserParams {
    #[cfg_attr(test, dummy(faker = "Name()"))]
    name: String,
}

impl HttpUpdateUserParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<HttpUpdateUserParams> for UpdateUserParams {
    fn from(val: HttpUpdateUserParams) -> Self {
        UpdateUserParams::new(val.name)
    }
}

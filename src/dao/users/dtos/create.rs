use chrono::Utc;
#[cfg(test)]
use fake::{faker::lorem::en::Word, faker::name::en::Name, Dummy};
use uuid::Uuid;

use super::entity::{CreateUserValidationError, User, UserAuthType};

#[cfg_attr(test, derive(Dummy, Debug, Clone, PartialEq, Eq))]
pub struct CreateUserParams {
    #[cfg_attr(test, dummy(faker = "Name()"))]
    name: String,
    auth_type: UserAuthType,
    #[cfg_attr(test, dummy(faker = "Word()"))]
    external_id: String,
}

impl CreateUserParams {
    pub fn new(name: String, auth_type: UserAuthType, external_id: String) -> Self {
        Self {
            name,
            auth_type,
            external_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn auth_type(&self) -> UserAuthType {
        self.auth_type
    }

    pub fn external_id(&self) -> &str {
        &self.external_id
    }
}

impl TryInto<User> for CreateUserParams {
    type Error = CreateUserValidationError;

    fn try_into(self) -> Result<User, Self::Error> {
        let now = Utc::now().naive_utc();

        User::new(
            Uuid::new_v4(),
            self.name,
            self.auth_type,
            self.external_id,
            now,
            now,
        )
    }
}

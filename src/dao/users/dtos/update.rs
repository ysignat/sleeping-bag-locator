#[cfg(test)]
use fake::{faker::name::en::Name, Dummy};

use super::entity::{UpdateUserValidationError, User};

#[cfg_attr(test, derive(Dummy, Clone, Debug, PartialEq, Eq))]
pub struct UpdateUserParams {
    #[cfg_attr(test, dummy(faker = "Name()"))]
    name: String,
}

impl UpdateUserParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl User {
    pub fn try_update(&mut self, value: UpdateUserParams) -> Result<(), UpdateUserValidationError> {
        self.set_name(value.name)
    }
}

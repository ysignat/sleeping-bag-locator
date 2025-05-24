#[cfg(test)]
use chrono::DateTime;
use chrono::{NaiveDateTime, Utc};
#[cfg(test)]
use fake::{
    faker::chrono::en::DateTimeBetween,
    faker::{lorem::en::Word, name::en::Name},
    Dummy,
    Fake,
    Faker,
    Rng,
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Copy)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum UserAuthType {
    Github,
}

#[cfg(test)]
impl Dummy<Faker> for UserAuthType {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Self::Github
    }
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct User {
    id: Uuid,
    name: String,
    auth_type: UserAuthType,
    external_id: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CreateUserValidationError {
    #[error("Empty name is not allowed")]
    NameIsEmpty,
    #[error("Name '{name:?}' is very long")]
    NameTooLong { name: String },
    #[error("Empty external ID is not allowed")]
    ExternalIdIsEmpty,
    #[error("External ID '{external_id:?}' is very long")]
    ExternalIdTooLong { external_id: String },
    #[error(
        "Last update time ({updated_at:?}) cannot be less than creation time ({created_at:?})"
    )]
    UpdatedBeforeCreation {
        updated_at: NaiveDateTime,
        created_at: NaiveDateTime,
    },
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum UpdateUserValidationError {
    #[error("Empty name is not allowed")]
    NameIsEmpty,
    #[error("Name '{name:?}' is very long")]
    NameTooLong { name: String },
}

impl User {
    const MAX_NAME_LENGTH: usize = 128;
    const MAX_EXTERNAL_ID_LENGTH: usize = 128;

    pub(super) fn new(
        id: Uuid,
        name: String,
        auth_type: UserAuthType,
        external_id: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Result<Self, CreateUserValidationError> {
        if name.is_empty() {
            return Err(CreateUserValidationError::NameIsEmpty);
        }
        if name.len().gt(&Self::MAX_NAME_LENGTH) {
            return Err(CreateUserValidationError::NameTooLong { name });
        }

        if external_id.is_empty() {
            return Err(CreateUserValidationError::ExternalIdIsEmpty);
        }
        if external_id.len().gt(&Self::MAX_EXTERNAL_ID_LENGTH) {
            return Err(CreateUserValidationError::ExternalIdTooLong { external_id });
        }

        if updated_at.lt(&created_at) {
            return Err(CreateUserValidationError::UpdatedBeforeCreation {
                updated_at,
                created_at,
            });
        }

        Ok(User {
            id,
            name,
            auth_type,
            external_id,
            created_at,
            updated_at,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn set_name(&mut self, name: String) -> Result<(), UpdateUserValidationError> {
        if name.is_empty() {
            return Err(UpdateUserValidationError::NameIsEmpty);
        }
        if name.len().gt(&Self::MAX_NAME_LENGTH) {
            return Err(UpdateUserValidationError::NameTooLong { name });
        }

        self.name = name;
        self.updated_at = Utc::now().naive_utc();

        Ok(())
    }

    pub fn auth_type(&self) -> UserAuthType {
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

#[cfg(test)]
impl Dummy<Faker> for User {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        let now = Utc::now();
        let created_at = DateTimeBetween(DateTime::<Utc>::MIN_UTC, now).fake::<DateTime<Utc>>();
        let updated_at = DateTimeBetween(created_at, DateTime::<Utc>::MAX_UTC)
            .fake::<DateTime<Utc>>()
            .naive_utc();

        Self::new(
            Faker.fake(),
            Name().fake(),
            Faker.fake(),
            Word().fake(),
            created_at.naive_utc(),
            updated_at,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;

    use super::*;

    #[test]
    fn name_validation() {
        let faked = Faker.fake::<User>();

        let err = User::new(
            faked.id,
            String::new(),
            faked.auth_type,
            faked.external_id.clone(),
            faked.created_at,
            faked.updated_at,
        );
        assert_eq!(err, Err(CreateUserValidationError::NameIsEmpty));

        let long: String = ((User::MAX_NAME_LENGTH + 1)..(User::MAX_NAME_LENGTH * 2)).fake();

        let err = User::new(
            faked.id,
            long.clone(),
            faked.auth_type,
            faked.external_id,
            faked.created_at,
            faked.updated_at,
        );

        assert_eq!(
            err,
            Err(CreateUserValidationError::NameTooLong { name: long })
        );
    }

    #[test]
    fn name_update() {
        let mut faked = Faker.fake::<User>();

        let err = faked.set_name(String::new());
        assert_eq!(err, Err(UpdateUserValidationError::NameIsEmpty));

        let long: String = ((User::MAX_NAME_LENGTH + 1)..(User::MAX_NAME_LENGTH * 2)).fake();

        let err = faked.set_name(long.clone());

        assert_eq!(
            err,
            Err(UpdateUserValidationError::NameTooLong { name: long })
        );

        let normal: String = Name().fake();

        faked.set_name(normal.clone()).unwrap();

        assert_eq!(faked.name(), normal);
        assert!(faked.created_at().lt(&faked.updated_at()));
    }

    #[test]
    fn external_id_validation() {
        let faked = Faker.fake::<User>();

        let err = User::new(
            faked.id,
            faked.name.clone(),
            faked.auth_type,
            String::new(),
            faked.created_at,
            faked.updated_at,
        );
        assert_eq!(err, Err(CreateUserValidationError::ExternalIdIsEmpty));

        let long: String =
            ((User::MAX_EXTERNAL_ID_LENGTH + 1)..(User::MAX_EXTERNAL_ID_LENGTH * 2)).fake();

        let err = User::new(
            faked.id,
            faked.name,
            faked.auth_type,
            long.clone(),
            faked.created_at,
            faked.updated_at,
        );

        assert_eq!(
            err,
            Err(CreateUserValidationError::ExternalIdTooLong { external_id: long })
        );
    }

    #[test]
    fn updated_before_creation() {
        let faked = Faker.fake::<User>();
        let now = Utc::now();
        let updated_at = DateTimeBetween(DateTime::<Utc>::MIN_UTC, now)
            .fake::<DateTime<Utc>>()
            .naive_utc();
        let created_at = DateTimeBetween(now, DateTime::<Utc>::MAX_UTC)
            .fake::<DateTime<Utc>>()
            .naive_utc();

        let err = User::new(
            faked.id,
            faked.name,
            faked.auth_type,
            faked.external_id,
            created_at,
            updated_at,
        );

        assert_eq!(
            err,
            Err(CreateUserValidationError::UpdatedBeforeCreation {
                updated_at,
                created_at
            })
        );
    }
}

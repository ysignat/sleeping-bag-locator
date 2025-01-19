use chrono::{NaiveDateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use super::params::{MutableParams, Params};

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Entity {
    id: Uuid,
    name: String,
    location: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Entity {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn try_mutate(self, mutation: &MutableParams) -> Result<Self, EntityBuilderError> {
        let now = Utc::now().naive_utc();
        let entity = EntityBuilder::new()
            .id(self.id)
            .name(mutation.name().to_owned())
            .location(mutation.location().to_owned())
            .created_at(self.created_at)
            .update_at(now)
            .build()?;

        Ok(entity)
    }
}

impl TryInto<Entity> for Params {
    type Error = EntityBuilderError;

    fn try_into(self) -> Result<Entity, Self::Error> {
        let entity = EntityBuilder::new()
            .location(self.location().to_owned())
            .name(self.name().to_owned())
            .build()?;
        Ok(entity)
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct EntityBuilder {
    id: Uuid,
    name: Option<String>,
    location: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum EntityBuilderError {
    #[error("Name was not set in builder")]
    NameNotSet,
    #[error("Empty name is not allowed")]
    NameIsEmpty,
    #[error("Name '{name:?}' is very long")]
    NameTooLong { name: String },
    #[error("Location was not set in builder")]
    LocationNotSet,
    #[error("Empty location is not allowed")]
    LocationIsEmpty,
    #[error("Location '{location:?}' is very long")]
    LocationTooLong { location: String },
    #[error(
        "Last update time ({updated_at:?}) cannot be less than creation time ({created_at:?})"
    )]
    UpdatedBeforeCreation {
        updated_at: NaiveDateTime,
        created_at: NaiveDateTime,
    },
}

impl Default for EntityBuilder {
    fn default() -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            name: None,
            location: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl EntityBuilder {
    const MAX_NAME_LENGTH: usize = 128;
    const MAX_LOCATION_LENGTH: usize = 128;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    pub fn created_at(mut self, created_at: NaiveDateTime) -> Self {
        self.created_at = created_at;
        self
    }

    pub fn update_at(mut self, updated_at: NaiveDateTime) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn build(self) -> Result<Entity, EntityBuilderError> {
        let name = self.name.ok_or(EntityBuilderError::NameNotSet)?;
        let location = self.location.ok_or(EntityBuilderError::LocationNotSet)?;

        if name.is_empty() {
            return Err(EntityBuilderError::NameIsEmpty);
        }
        if name.len().gt(&Self::MAX_NAME_LENGTH) {
            return Err(EntityBuilderError::NameTooLong { name });
        }

        if location.is_empty() {
            return Err(EntityBuilderError::LocationIsEmpty);
        }
        if location.len().gt(&Self::MAX_LOCATION_LENGTH) {
            return Err(EntityBuilderError::LocationTooLong { location });
        }

        if self.updated_at.lt(&self.created_at) {
            return Err(EntityBuilderError::UpdatedBeforeCreation {
                updated_at: self.updated_at,
                created_at: self.created_at,
            });
        }

        Ok(Entity {
            id: self.id,
            name,
            location,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use fake::{
        faker::{address::en::CityName, lorem::en::Word},
        uuid::UUIDv4,
        Fake,
    };

    use super::*;

    #[test]
    fn default_builder() {
        let builder = EntityBuilder::default();
        println!("{builder:#?}");

        assert_eq!(builder.location, None);
        assert_eq!(builder.name, None);
    }

    #[test]
    fn location_not_set() {
        let name = Word().fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(EntityBuilderError::LocationNotSet));
    }

    #[test]
    fn name_not_set() {
        let location = CityName().fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(EntityBuilderError::NameNotSet));
    }

    #[test]
    fn empty_location() {
        let builder = EntityBuilder::default();
        let name = Word().fake();
        println!("{builder:#?}");
        let builder_err = builder.location(String::new()).name(name).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(EntityBuilderError::LocationIsEmpty));
    }

    #[test]
    fn empty_name() {
        let builder = EntityBuilder::default();
        let location = CityName().fake();
        println!("{builder:#?}");
        let builder_err = builder.name(String::new()).location(location).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(EntityBuilderError::NameIsEmpty));
    }

    #[test]
    fn long_location() {
        let location: String = ((EntityBuilder::MAX_LOCATION_LENGTH + 1)
            ..(EntityBuilder::MAX_LOCATION_LENGTH * 2))
            .fake();
        let name = Word().fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location.clone()).name(name).build();
        println!("{builder_err:#?}");

        assert_eq!(
            builder_err,
            Err(EntityBuilderError::LocationTooLong { location })
        );
    }

    #[test]
    fn long_name() {
        let name: String = ((EntityBuilder::MAX_LOCATION_LENGTH + 1)
            ..(EntityBuilder::MAX_LOCATION_LENGTH * 2))
            .fake();
        let location = CityName().fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name.clone()).location(location).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(EntityBuilderError::NameTooLong { name }));
    }

    #[test]
    fn id_and_time_not_set() {
        let location = CityName().fake();
        let name = Word().fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_ok = builder.name(name).location(location).build();
        println!("{builder_ok:#?}");

        assert!(builder_ok.is_ok());
    }

    #[test]
    fn updated_before_creation() {
        let location = CityName().fake();
        let name = Word().fake();
        let updated_at = Utc::now().naive_utc();
        sleep(Duration::from_secs(1));
        let created_at = Utc::now().naive_utc();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder
            .name(name)
            .location(location)
            .created_at(created_at)
            .update_at(updated_at)
            .build();
        println!("{builder_err:#?}");

        assert_eq!(
            builder_err,
            Err(EntityBuilderError::UpdatedBeforeCreation {
                updated_at,
                created_at
            })
        );
    }

    #[test]
    fn all_set() {
        let location = CityName().fake();
        let name = Word().fake();
        let created_at = Utc::now().naive_utc();
        sleep(Duration::from_secs(1));
        let updated_at = Utc::now().naive_utc();
        let id = UUIDv4.fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let entity = builder
            .id(id)
            .name(name)
            .location(location)
            .created_at(created_at)
            .update_at(updated_at)
            .build()
            .unwrap();
        println!("{entity:#?}");
    }
}

use anyhow::anyhow;
use chrono::{NaiveDateTime, Utc};
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

    pub fn try_mutate(self, mutation: &MutableParams) -> anyhow::Result<Self> {
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
    type Error = anyhow::Error;

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

    pub fn build(self) -> anyhow::Result<Entity> {
        let name = self.name.ok_or(anyhow!("Name was not set in builder"))?;
        let location = self
            .location
            .ok_or(anyhow!("Location was not set in builder"))?;

        if name.is_empty() {
            return Err(anyhow!("Empty name is not allowed"));
        }
        if name.len().gt(&Self::MAX_NAME_LENGTH) {
            return Err(anyhow!("Name is very long"));
        }

        if location.is_empty() {
            return Err(anyhow!("Empty location is not allowed"));
        }
        if location.len().gt(&Self::MAX_LOCATION_LENGTH) {
            return Err(anyhow!("Location is very long"));
        }

        if self.updated_at.lt(&self.created_at) {
            return Err(anyhow!(
                "Last update time cannot be less than creation time"
            ));
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
        let location = CityName().fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn name_not_set() {
        let name = Word().fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn empty_location() {
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(String::new()).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn empty_name() {
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(String::new()).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn long_location() {
        let location: String = ((EntityBuilder::MAX_LOCATION_LENGTH + 1)
            ..(EntityBuilder::MAX_LOCATION_LENGTH * 2))
            .fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn long_name() {
        let name: String = ((EntityBuilder::MAX_LOCATION_LENGTH + 1)
            ..(EntityBuilder::MAX_LOCATION_LENGTH * 2))
            .fake();
        let builder = EntityBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
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
    fn created_later_than_updated() {
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

        assert!(builder_err.is_err());
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

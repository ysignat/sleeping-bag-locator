use chrono::Utc;
#[cfg(test)]
use fake::{faker::address::en::CityName, faker::lorem::en::Word, Dummy};
use thiserror::Error;

use super::item::{Item, ItemBuilder, ItemBuilderError};

#[cfg_attr(test, derive(Dummy, Clone, Debug, PartialEq, Eq))]
pub struct UpdateItemParams {
    #[cfg_attr(test, dummy(faker = "Word()"))]
    name: String,
    #[cfg_attr(test, dummy(faker = "CityName()"))]
    location: String,
}

impl Item {
    pub fn try_update(self, mutation: &UpdateItemParams) -> Result<Self, ItemBuilderError> {
        let now = Utc::now().naive_utc();
        let entity = ItemBuilder::new()
            .id(self.id())
            .name(mutation.name().to_owned())
            .location(mutation.location().to_owned())
            .created_at(self.created_at())
            .update_at(now)
            .build()?;

        Ok(entity)
    }
}

impl TryInto<Item> for UpdateItemParams {
    type Error = ItemBuilderError;

    fn try_into(self) -> Result<Item, Self::Error> {
        let entity = ItemBuilder::new()
            .location(self.location().to_owned())
            .name(self.name().to_owned())
            .build()?;
        Ok(entity)
    }
}

impl UpdateItemParams {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct UpdateItemParamsBuilder {
    name: Option<String>,
    location: Option<String>,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum UpdateItemParamsBuilderError {
    #[error("Name is not set")]
    NameNotSet,
    #[error("Location is not set")]
    LocationNotSet,
}

impl UpdateItemParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    pub fn build(self) -> Result<UpdateItemParams, UpdateItemParamsBuilderError> {
        Ok(UpdateItemParams {
            name: self.name.ok_or(UpdateItemParamsBuilderError::NameNotSet)?,
            location: self
                .location
                .ok_or(UpdateItemParamsBuilderError::LocationNotSet)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;

    use super::*;

    #[test]
    fn name_not_set() {
        let location = CityName().fake();
        let builder = UpdateItemParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(UpdateItemParamsBuilderError::NameNotSet));
    }

    #[test]
    fn location_not_set() {
        let name = Word().fake();
        let builder = UpdateItemParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name).build();
        println!("{builder_err:#?}");

        assert_eq!(
            builder_err,
            Err(UpdateItemParamsBuilderError::LocationNotSet)
        );
    }

    #[test]
    fn ok() {
        let location: String = CityName().fake();
        let name: String = Word().fake();
        let builder = UpdateItemParamsBuilder::default();
        println!("{builder:#?}");
        let params = builder
            .location(location.clone())
            .name(name.clone())
            .build()
            .unwrap();
        println!("{params:#?}");

        assert_eq!(params, UpdateItemParams { name, location });
    }
}

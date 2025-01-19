#[cfg(test)]
use fake::{faker::address::en::CityName, faker::lorem::en::Word, Dummy};
use thiserror::Error;

use super::item::{Item, ItemBuilder, ItemBuilderError};

#[cfg_attr(test, derive(Dummy, Clone, PartialEq, Eq))]
#[derive(Debug)]
pub struct CreateItemParams {
    #[cfg_attr(test, dummy(faker = "Word()"))]
    name: String,
    #[cfg_attr(test, dummy(faker = "CityName()"))]
    location: String,
}

impl CreateItemParams {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}

impl TryInto<Item> for CreateItemParams {
    type Error = ItemBuilderError;

    fn try_into(self) -> Result<Item, Self::Error> {
        let entity = ItemBuilder::new()
            .location(self.location().to_owned())
            .name(self.name().to_owned())
            .build()?;
        Ok(entity)
    }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct CreateItemsParamsBuilder {
    name: Option<String>,
    location: Option<String>,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CreateItemParamsBuilderError {
    #[error("Name is not set")]
    NameNotSet,
    #[error("Location is not set")]
    LocationNotSet,
}

impl CreateItemsParamsBuilder {
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

    pub fn build(self) -> Result<CreateItemParams, CreateItemParamsBuilderError> {
        Ok(CreateItemParams {
            name: self.name.ok_or(CreateItemParamsBuilderError::NameNotSet)?,
            location: self
                .location
                .ok_or(CreateItemParamsBuilderError::LocationNotSet)?,
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
        let builder = CreateItemsParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(CreateItemParamsBuilderError::NameNotSet));
    }

    #[test]
    fn location_not_set() {
        let name = Word().fake();
        let builder = CreateItemsParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name).build();
        println!("{builder_err:#?}");

        assert_eq!(
            builder_err,
            Err(CreateItemParamsBuilderError::LocationNotSet)
        );
    }

    #[test]
    fn ok() {
        let location: String = CityName().fake();
        let name: String = Word().fake();
        let builder = CreateItemsParamsBuilder::default();
        println!("{builder:#?}");
        let params = builder
            .location(location.clone())
            .name(name.clone())
            .build()
            .unwrap();
        println!("{params:#?}");

        assert_eq!(params, CreateItemParams { name, location });
    }
}

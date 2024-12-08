use anyhow::{anyhow, Result};
#[cfg(test)]
use fake::{faker::address::en::CityName, faker::lorem::en::Word, Dummy};

#[cfg_attr(test, derive(Dummy, Clone, Debug, PartialEq, Eq))]
pub struct MutableParams {
    #[cfg_attr(test, dummy(faker = "Word()"))]
    name: String,
    #[cfg_attr(test, dummy(faker = "CityName()"))]
    location: String,
}

impl MutableParams {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct MutableParamsBuilder {
    name: Option<String>,
    location: Option<String>,
}

impl MutableParamsBuilder {
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

    pub fn build(self) -> Result<MutableParams> {
        Ok(MutableParams {
            name: self.name.ok_or(anyhow!("Name is not set"))?,
            location: self.location.ok_or(anyhow!("Location is not set"))?,
        })
    }
}

#[cfg_attr(test, derive(Dummy, Clone, Debug, PartialEq, Eq))]
pub struct Params {
    #[cfg_attr(test, dummy(faker = "Word()"))]
    name: String,
    #[cfg_attr(test, dummy(faker = "CityName()"))]
    location: String,
}

impl Params {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct ParamsBuilder {
    name: Option<String>,
    location: Option<String>,
}

impl ParamsBuilder {
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

    pub fn build(self) -> Result<Params> {
        Ok(Params {
            name: self.name.ok_or(anyhow!("Name is not set"))?,
            location: self.location.ok_or(anyhow!("Location is not set"))?,
        })
    }
}

#[cfg(test)]
mod tests_params {
    use fake::Fake;

    use super::*;

    #[test]
    fn name_not_set() {
        let location = CityName().fake();
        let builder = ParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn location_not_set() {
        let name = Word().fake();
        let builder = ParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn ok() {
        let location: String = CityName().fake();
        let name: String = Word().fake();
        let builder = ParamsBuilder::default();
        println!("{builder:#?}");
        let params = builder
            .location(location.clone())
            .name(name.clone())
            .build()
            .unwrap();
        println!("{params:#?}");

        assert_eq!(params, Params { name, location });
    }
}

#[cfg(test)]
mod tests_mutable_params {
    use fake::Fake;

    use super::*;

    #[test]
    fn name_not_set() {
        let location = CityName().fake();
        let builder = MutableParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.location(location).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn location_not_set() {
        let name = Word().fake();
        let builder = MutableParamsBuilder::default();
        println!("{builder:#?}");
        let builder_err = builder.name(name).build();
        println!("{builder_err:#?}");

        assert!(builder_err.is_err());
    }

    #[test]
    fn ok() {
        let location: String = CityName().fake();
        let name: String = Word().fake();
        let builder = MutableParamsBuilder::default();
        println!("{builder:#?}");
        let params = builder
            .location(location.clone())
            .name(name.clone())
            .build()
            .unwrap();
        println!("{params:#?}");

        assert_eq!(params, MutableParams { name, location });
    }
}

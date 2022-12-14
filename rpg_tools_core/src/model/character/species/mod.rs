use crate::model::character::species::appearance::AppearanceOptions;
use crate::model::character::species::gender::GenderOption;
use crate::model::name::Name;
use anyhow::{Context, Result};

pub mod appearance;
pub mod gender;
pub mod manager;

/// The id of a [`Species`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SpeciesId(usize);

impl SpeciesId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

impl From<usize> for SpeciesId {
    fn from(value: usize) -> Self {
        SpeciesId::new(value)
    }
}

/// The species of a [`character`](crate::model::character::Character).
#[derive(Clone, Debug, PartialEq)]
pub struct Species {
    id: SpeciesId,
    name: Name,
    gender_option: GenderOption,
    appearance: AppearanceOptions,
}

impl Species {
    pub fn new<I: Into<SpeciesId>, S: Into<String>>(
        id: I,
        name: S,
        gender_option: GenderOption,
        appearance: AppearanceOptions,
    ) -> Result<Self> {
        let id = id.into();
        let name = name.into();
        let name = Name::new(name).with_context(|| format!("Failed to create species {}", id.0))?;

        Ok(Self {
            id,
            name,
            gender_option,
            appearance,
        })
    }

    pub fn id(&self) -> SpeciesId {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn gender_option(&self) -> GenderOption {
        self.gender_option
    }

    pub fn appearance(&self) -> &AppearanceOptions {
        &self.appearance
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use GenderOption::*;

    #[test]
    fn test_new() {
        let appearance = AppearanceOptions::default();
        assert!(Species::new(0, "Test", TwoGenders, appearance.clone()).is_ok());
        assert!(Species::new(SpeciesId::new(2), "Test2", NoGender, appearance).is_ok());
    }

    #[test]
    fn test_invalid_name() {
        let appearance = AppearanceOptions::default();
        assert!(Species::new(0, "", TwoGenders, appearance).is_err());
    }
}

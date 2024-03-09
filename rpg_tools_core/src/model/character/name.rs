use crate::model::name::Name;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A name of a [`character`](crate::model::character::Character).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CharacterName {
    first: Name,
    middle: Option<Name>,
    last: Lastname,
}

impl CharacterName {
    /// Returns a character name with middle or last name.
    pub fn only_first(first: Name) -> Self {
        Self {
            first,
            middle: None,
            last: Lastname::None,
        }
    }

    /// Returns a character name without middle name.
    pub fn simple(first: Name, last: Lastname) -> Self {
        Self {
            first,
            middle: None,
            last,
        }
    }

    /// Returns a character name with middle name.
    pub fn full(first: Name, middle: Name, last: Lastname) -> Self {
        Self {
            first,
            middle: Some(middle),
            last,
        }
    }

    /// Returns a character name with middle name.
    pub fn parse<S: Into<String>>(first: S, middle: S, last: S, last_type: S) -> Result<Self> {
        if let Some(first) = Name::new(first) {
            let last_type: String = last_type.into();

            let last_name = if last_type.eq("None") {
                Lastname::None
            } else if let Some(name) = Name::new(last) {
                match last_type.as_str() {
                    "Family" => Lastname::Family(name),
                    "Patronymic" => Lastname::Patronymic(name),
                    "Matronymic" => Lastname::Matronymic(name),
                    _ => return bail!("Unknown type of last name"),
                }
            } else {
                return bail!("Last name is invalid");
            };

            Ok(Self {
                first,
                middle: Name::new(middle),
                last: last_name,
            })
        } else {
            bail!("First name is invalid")
        }
    }

    pub fn first(&self) -> &Name {
        &self.first
    }

    pub fn middle(&self) -> Option<&Name> {
        match &self.middle {
            Some(middle) => Some(middle),
            None => None,
        }
    }

    pub fn last(&self) -> &Lastname {
        &self.last
    }
}

impl Display for CharacterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.first.str())?;

        if let Some(middle) = &self.middle {
            write!(f, " {}", middle.str())?;
        }

        if let Lastname::Family(name) | Lastname::Patronymic(name) | Lastname::Matronymic(name) =
            &self.last
        {
            write!(f, " {}", name.str())?;
        }

        Ok(())
    }
}

/// The last name of a [`character`](crate::model::character::Character).
#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Lastname {
    #[default]
    None,
    Family(Name),
    /// A last name based on the first name of the father.
    Patronymic(Name),
    /// A last name based on the first name of the mother.
    Matronymic(Name),
}

impl Lastname {
    pub fn name(&self) -> Option<&Name> {
        match &self {
            Lastname::None => None,
            Lastname::Family(name) | Lastname::Patronymic(name) | Lastname::Matronymic(name) => {
                Some(name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_name_to_string() {
        let first = Name::new("A").unwrap();
        let middle = Name::new("B").unwrap();
        let last = Lastname::Family(Name::new("C").unwrap());
        let full = CharacterName::full(first, middle, last);

        assert_eq!(full.to_string(), "A B C");
    }

    #[test]
    fn simple_name_to_string() {
        let first = Name::new("First").unwrap();
        let last = Lastname::Family(Name::new("Last").unwrap());
        let simple = CharacterName::simple(first, last);

        assert_eq!(simple.to_string(), "First Last");
    }

    #[test]
    fn parse_success() {
        let name = CharacterName::parse("A", "B", "C", "Matronymic").unwrap();
        let desired = CharacterName::full(
            Name::new("A").unwrap(),
            Name::new("B").unwrap(),
            Lastname::Matronymic(Name::new("C").unwrap()),
        );

        assert_eq!(desired, name);
    }
}

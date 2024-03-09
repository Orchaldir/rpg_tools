use crate::model::name::Name;
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
    /// Returns a character name with middle name.
    pub fn full(first: Name, middle: Name, last: Lastname) -> Self {
        Self {
            first,
            middle: Some(middle),
            last,
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
}

impl Display for CharacterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.first.fmt(f)?;

        if let Some(middle) = &self.middle {
            write!(f, " {}", middle.to_string())?;
        }

        if let Lastname::Family(name) | Lastname::Patronymic(name) | Lastname::Matronymic(name) =
            &self.last
        {
            write!(f, " {}", name.to_string())?;
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
}

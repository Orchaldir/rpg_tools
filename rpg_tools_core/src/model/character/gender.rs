use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The gender of a [`character`](crate::model::character::Character).
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    #[default]
    Female,
    Genderless,
    Male,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Gender {
    fn from(string: &str) -> Self {
        match string {
            "Female" => Self::Female,
            "Genderless" => Self::Genderless,
            "Male" => Self::Male,
            _ => Self::default(),
        }
    }
}

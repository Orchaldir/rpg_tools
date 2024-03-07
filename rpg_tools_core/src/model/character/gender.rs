use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The gender of the [`character`](crate::model::character::Character).
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

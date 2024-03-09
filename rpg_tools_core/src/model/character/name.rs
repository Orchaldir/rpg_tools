use crate::model::name::Name;
use serde::{Deserialize, Serialize};

/// A name of a [`character`](crate::model::character::Character).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CharacterName {
    first: Name,
    middle: Option<Name>,
    last: Lastname,
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

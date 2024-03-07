use serde::{Deserialize, Serialize};

/// The gender of the [`character`](crate::model::character::Character).
#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    #[default]
    Female,
    Genderless,
    Male,
}

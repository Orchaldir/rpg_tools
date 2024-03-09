use crate::model::character::culture::CultureId;
use crate::model::character::gender::Gender;
use crate::model::character::name::CharacterName;
use crate::model::name::Name;
use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};

pub mod culture;
pub mod gender;
pub mod name;

/// The unique identifier of a [`character`](Character).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CharacterId(usize);

impl Id for CharacterId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A character in the game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character {
    id: CharacterId,
    pub name: CharacterName,
    pub gender: Gender,
    pub culture: CultureId,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: CharacterName::only_first(Name::new(format!("Character {}", id.0)).unwrap()),
            gender: Gender::default(),
            culture: Default::default(),
        }
    }
}

impl Element<CharacterId> for Character {
    fn id(&self) -> CharacterId {
        self.id
    }

    fn with_id(self, id: CharacterId) -> Self {
        Character { id, ..self }
    }
}

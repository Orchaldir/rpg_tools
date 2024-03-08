use crate::model::character::gender::Gender;
use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};

pub mod culture;
pub mod gender;

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
    name: String,
    pub gender: Gender,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: format!("Character {}", id.0),
            gender: Gender::default(),
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

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

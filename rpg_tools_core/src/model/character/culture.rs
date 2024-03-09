use crate::model::name::{EditableName, Name};
use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};

/// The unique identifier of a [`culture`](Culture).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CultureId(usize);

impl Id for CultureId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A culture in the game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Culture {
    id: CultureId,
    name: Name,
}

impl Culture {
    pub fn new(id: CultureId) -> Self {
        Culture {
            id,
            name: Name::new(format!("Culture {}", id.0)).unwrap(),
        }
    }
}

impl Element<CultureId> for Culture {
    fn id(&self) -> CultureId {
        self.id
    }

    fn with_id(self, id: CultureId) -> Self {
        Culture { id, ..self }
    }
}

impl EditableName for Culture {
    fn name(&self) -> &Name {
        &self.name
    }
    fn set_name(&mut self, name: Name) {
        self.name = name;
    }
}

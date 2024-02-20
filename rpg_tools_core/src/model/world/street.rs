use crate::model::world::town::TownId;
use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The unique identifier of a [`street`](Street).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct StreetId(usize);

impl Id for StreetId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A street in the game.
#[derive(Clone, Debug, PartialEq)]
pub struct Street {
    id: StreetId,
    name: String,
    pub towns: HashSet<TownId>,
}

impl Element<StreetId> for Street {
    fn new(id: StreetId) -> Self {
        Street {
            id,
            name: format!("Street {}", id.0),
            towns: HashSet::new(),
        }
    }

    fn id(&self) -> StreetId {
        self.id
    }

    fn with_id(self, id: StreetId) -> Self {
        Street { id, ..self }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

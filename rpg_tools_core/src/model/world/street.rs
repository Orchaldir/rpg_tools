use crate::model::world::town::towns::WithTowns;
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Street {
    id: StreetId,
    name: String,
    towns: HashSet<TownId>,
}

impl Street {
    pub fn new(id: StreetId) -> Self {
        Street {
            id,
            name: format!("Street {}", id.0),
            towns: HashSet::new(),
        }
    }
}

impl Element<StreetId> for Street {
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

impl WithTowns for Street {
    fn towns(&self) -> &HashSet<TownId> {
        &self.towns
    }

    fn towns_mut(&mut self) -> &mut HashSet<TownId> {
        &mut self.towns
    }
}

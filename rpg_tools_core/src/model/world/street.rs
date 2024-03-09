use crate::model::name::{EditableName, Name};
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
    name: Name,
    towns: HashSet<TownId>,
}

impl Street {
    pub fn new(id: StreetId) -> Self {
        Street {
            id,
            name: Name::new(format!("Street {}", id.0)).unwrap(),
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
}

impl EditableName for Street {
    fn name(&self) -> &Name {
        &self.name
    }
    fn set_name(&mut self, name: Name) {
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

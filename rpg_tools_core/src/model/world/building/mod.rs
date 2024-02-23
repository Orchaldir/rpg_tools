pub mod lot;

use crate::model::world::building::lot::BuildingLot;
use crate::utils::storage::{Element, Id};

/// The unique identifier of a [`building`](Building).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BuildingId(usize);

impl Id for BuildingId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A building in the game.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Building {
    id: BuildingId,
    name: String,
    lot: BuildingLot,
}

impl Building {
    pub fn new(id: BuildingId, lot: BuildingLot) -> Self {
        Building {
            id,
            name: format!("Building {}", id.0),
            lot,
        }
    }
}

impl Element<BuildingId> for Building {
    fn id(&self) -> BuildingId {
        self.id
    }

    fn with_id(self, id: BuildingId) -> Self {
        Building { id, ..self }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

use crate::model::name::{EditableName, Name};
use crate::model::world::town::towns::WithTowns;
use crate::model::world::town::TownId;
use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The unique identifier of a [`mountain`](Mountain).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MountainId(usize);

impl Id for MountainId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A mountain or hill in the game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Mountain {
    id: MountainId,
    name: Name,
    towns: HashSet<TownId>,
}

impl Mountain {
    pub fn new(id: MountainId) -> Self {
        Mountain {
            id,
            name: Name::new(format!("Mountain {}", id.0)).unwrap(),
            towns: Default::default(),
        }
    }
}

impl Element<MountainId> for Mountain {
    fn id(&self) -> MountainId {
        self.id
    }

    fn with_id(self, id: MountainId) -> Self {
        Mountain { id, ..self }
    }
}

impl EditableName for Mountain {
    fn name(&self) -> &Name {
        &self.name
    }
    fn set_name(&mut self, name: Name) {
        self.name = name;
    }
}

impl WithTowns for Mountain {
    fn towns(&self) -> &HashSet<TownId> {
        &self.towns
    }

    fn towns_mut(&mut self) -> &mut HashSet<TownId> {
        &mut self.towns
    }
}

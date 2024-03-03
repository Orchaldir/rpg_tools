use crate::model::world::town::towns::WithTowns;
use crate::model::world::town::TownId;
use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The unique identifier of a [`river`](River).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RiverId(usize);

impl Id for RiverId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A river in the game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct River {
    id: RiverId,
    name: String,
    pub towns: HashSet<TownId>,
}

impl River {
    pub fn new(id: RiverId) -> Self {
        River {
            id,
            name: format!("River {}", id.0),
            towns: HashSet::new(),
        }
    }
}

impl Element<RiverId> for River {
    fn id(&self) -> RiverId {
        self.id
    }

    fn with_id(self, id: RiverId) -> Self {
        River { id, ..self }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl WithTowns for River {
    fn towns(&self) -> &HashSet<TownId> {
        &self.towns
    }

    fn towns_mut(&mut self) -> &mut HashSet<TownId> {
        &mut self.towns
    }
}

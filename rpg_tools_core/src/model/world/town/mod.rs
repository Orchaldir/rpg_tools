pub mod terrain;
pub mod tile;

use crate::model::math::size2d::Size2d;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::tile::TownTile;
use crate::utils::map::border::BorderMap;
use crate::utils::storage::{Element, Id};

/// The unique identifier of a [`town`](Town).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TownId(usize);

impl Id for TownId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A town in the game.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Town {
    id: TownId,
    name: String,
    pub map: BorderMap<TownTile, bool>,
}

impl Town {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Element<TownId> for Town {
    fn new(id: TownId) -> Self {
        Town {
            id,
            name: format!("Town {}", id.0),
            map: BorderMap::simple(Size2d::square(1), TownTile::new(Terrain::Plain), false),
        }
    }

    fn id(&self) -> TownId {
        self.id
    }

    fn with_id(self, id: TownId) -> Self {
        Town { id, ..self }
    }
}

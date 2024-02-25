pub mod construction;
pub mod terrain;
pub mod tile;

use crate::model::math::size2d::Size2d;
use crate::model::world::building::lot::BuildingLot;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::tile::TownTile;
use crate::utils::map::tile::TileMap;
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
    pub map: TileMap<TownTile>,
}

impl Town {
    pub fn new(id: TownId) -> Self {
        Town {
            id,
            name: format!("Town {}", id.0),
            map: TileMap::simple(Size2d::square(1), TownTile::new(Terrain::Plain)),
        }
    }

    pub fn is_lot_construction(&self, lot: &BuildingLot, construction: Construction) -> bool {
        let start_x = self.map.get_size().to_x(lot.tile);
        let start_y = self.map.get_size().to_y(lot.tile);

        for y in start_y..(start_y + lot.size.height() as i32) {
            for x in start_x..(start_x + lot.size.height() as i32) {
                if let Some(tile) = self
                    .map
                    .get_size()
                    .to_index(x as u32, y as u32)
                    .and_then(|index| self.map.get_tile(index))
                {
                    if !tile.construction.eq(&construction) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Element<TownId> for Town {
    fn id(&self) -> TownId {
        self.id
    }

    fn with_id(self, id: TownId) -> Self {
        Town { id, ..self }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

pub mod construction;
pub mod terrain;
pub mod tile;

use crate::model::math::size2d::Size2d;
use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::BuildingId;
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

    pub fn simple(id: TownId, size: Size2d) -> Self {
        Town {
            id,
            name: format!("Town {}", id.0),
            map: TileMap::simple(size, TownTile::new(Terrain::Plain)),
        }
    }

    /// Tries to set the [`construction`](Construction) of a [`lot`](BuildingLot).
    /// Fails if part of the lot is outside the map or already has a construction.
    ///
    /// This can leave the town in an inconsistent state!
    pub fn set_lot_construction(&mut self, lot: &BuildingLot, construction: Construction) -> bool {
        let start_x = self.map.get_size().to_x(lot.tile);
        let start_y = self.map.get_size().to_y(lot.tile);

        for y in start_y..(start_y + lot.size.height()) {
            for x in start_x..(start_x + lot.size.width()) {
                if let Some(tile) = self
                    .map
                    .get_size()
                    .to_index(x, y)
                    .and_then(|index| self.map.get_tile_mut(index))
                {
                    if tile.construction != Construction::None {
                        return false;
                    }

                    tile.construction = construction.clone();
                } else {
                    return false;
                }
            }
        }

        true
    }

    fn check_lot_construction<F: Fn(&Construction) -> bool>(
        &self,
        lot: &BuildingLot,
        check: F,
    ) -> bool {
        let start_x = self.map.get_size().to_x(lot.tile);
        let start_y = self.map.get_size().to_y(lot.tile);

        for y in start_y..(start_y + lot.size.height()) {
            for x in start_x..(start_x + lot.size.width()) {
                if let Some(tile) = self
                    .map
                    .get_size()
                    .to_index(x, y)
                    .and_then(|index| self.map.get_tile(index))
                {
                    if !check(&tile.construction) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }

    /// Checks if the [`tiles`](TownTile) of the [`lot`](BuildingLot) match the [`construction`](Construction).
    pub fn is_lot_construction(&self, lot: &BuildingLot, construction: &Construction) -> bool {
        self.check_lot_construction(lot, |c| c.eq(construction))
    }

    /// Checks if the [`tiles`](TownTile) of the [`lot`](BuildingLot) are free or match the [`construction`](Construction).
    pub fn can_update_building(&self, lot: &BuildingLot, id: BuildingId) -> bool {
        let construction = Construction::Building { id };
        self.check_lot_construction(lot, |c| c.eq(&Construction::None) || c.eq(&construction))
    }

    /// Checks if the [`tiles`](TownTile) of the [`lot`](BuildingLot) are free.
    pub fn is_lot_free(&self, lot: &BuildingLot) -> bool {
        self.is_lot_construction(lot, &Construction::None)
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

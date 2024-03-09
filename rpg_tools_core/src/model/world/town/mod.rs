pub mod construction;
pub mod terrain;
pub mod tile;
pub mod towns;

use crate::model::math::size2d::Size2d;
use crate::model::name::{Name, WithName};
use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::BuildingId;
use crate::model::world::street::StreetId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::tile::TownTile;
use crate::utils::map::tile::TileMap;
use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};

/// The unique identifier of a [`town`](Town).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Town {
    id: TownId,
    name: Name,
    pub map: TileMap<TownTile>,
}

impl Town {
    pub fn new(id: TownId) -> Self {
        Self::simple(id, Size2d::square(1))
    }

    pub fn simple(id: TownId, size: Size2d) -> Self {
        Town {
            id,
            name: Name::new(format!("Town {}", id.0)).unwrap(),
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
        let is_building = construction.is_present();

        for y in start_y..(start_y + lot.size.height()) {
            for x in start_x..(start_x + lot.size.width()) {
                if let Some(tile) = self
                    .map
                    .get_size()
                    .to_index(x, y)
                    .and_then(|index| self.map.get_tile_mut(index))
                {
                    if is_building && tile.construction.is_present() {
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

    /// Checks the [`construction`](Construction) of the [`tiles`](TownTile) of the [`lot`](BuildingLot).
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

    /// Checks the [`construction`](Construction) of a [`tile`](TownTile).
    pub fn check_construction<F: Fn(&Construction) -> bool>(&self, index: usize, check: F) -> bool {
        self.map
            .get_tile(index)
            .map(|tile| check(&tile.construction))
            .unwrap_or(false)
    }

    /// Checks the [`construction`](Construction) of a [`tile`](TownTile).
    pub fn check_construction_xy<F: Fn(&Construction) -> bool>(
        &self,
        x: i32,
        y: i32,
        check: F,
    ) -> bool {
        self.map
            .get_tile_xy(x, y)
            .map(|tile| check(&tile.construction))
            .unwrap_or(false)
    }

    /// Does the town contain a specific street?
    pub fn contains_street(&self, id: StreetId) -> bool {
        self.map
            .get_tiles()
            .iter()
            .any(|tile| tile.construction.is_street(id))
    }

    /// Does the town contain a specific [`terrain`](Terrain)?
    pub fn contains_terrain(&self, terrain: Terrain) -> bool {
        self.map
            .get_tiles()
            .iter()
            .any(|tile| tile.terrain.eq(&terrain))
    }
}

impl Element<TownId> for Town {
    fn id(&self) -> TownId {
        self.id
    }

    fn with_id(self, id: TownId) -> Self {
        Town { id, ..self }
    }
}

impl WithName for Town {
    fn name(&self) -> &Name {
        &self.name
    }
    fn set_name(&mut self, name: Name) {
        self.name = name;
    }
}

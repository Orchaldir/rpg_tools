use crate::model::math::size2d::Size2d;
use crate::model::world::town::TownId;
use serde::{Deserialize, Serialize};

/// The lot, plot or parcel of a [`building`](crate::model::world::building::Building).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuildingLot {
    pub town: TownId,
    pub tile: usize,
    pub size: Size2d,
}

impl BuildingLot {
    pub fn new(town: TownId, tile: usize) -> Self {
        BuildingLot {
            town,
            tile,
            size: Size2d::square(1),
        }
    }

    pub fn big(town: TownId, tile: usize, size: Size2d) -> Self {
        BuildingLot { town, tile, size }
    }

    pub fn tile(tile: usize) -> BuildingLot {
        Self::new(TownId::default(), tile)
    }
}

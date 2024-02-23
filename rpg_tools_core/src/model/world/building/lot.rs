use crate::model::math::size2d::Size2d;
use crate::model::world::town::TownId;

/// A lot of a [`building`](crate::model::world::building::Building).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildingLot {
    pub town: TownId,
    pub tile: usize,
    pub size: Size2d,
}

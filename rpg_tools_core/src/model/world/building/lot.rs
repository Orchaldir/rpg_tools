use crate::model::world::town::TownId;

/// The lot, plot or parcel of a [`building`](crate::model::world::building::Building).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildingLot {
    pub town: TownId,
    pub tile: usize,
}

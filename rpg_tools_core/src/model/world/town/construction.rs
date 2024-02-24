use crate::model::world::building::BuildingId;
use crate::model::world::street::StreetId;

/// What is build on a [`tile`](crate::model::world::town::tile::TownTile)
/// of the [`town`](crate::model::world::town::Town)?
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Construction {
    Building { id: BuildingId },
    Street { id: StreetId },
    None,
}

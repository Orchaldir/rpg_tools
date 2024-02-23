use crate::model::world::building::BuildingId;

/// What is build on a [`tile`](crate::model::world::town::tile::TownTile)
/// of the [`town`](crate::model::world::town::Town)?
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Construction {
    Building { id: BuildingId },
    None,
}

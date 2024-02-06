use crate::model::world::town::terrain::Terrain;

/// A tile of the [`town`](crate::model::world::town::Town) map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TownTile {
    pub terrain: Terrain,
}

impl TownTile {
    pub fn new(terrain: Terrain) -> Self {
        Self { terrain }
    }
}

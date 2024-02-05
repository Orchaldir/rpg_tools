use crate::model::world::town::terrain::Terrain;

/// A cell of the [`town`](crate::model::world::town::Town) map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TownCell {
    pub terrain: Terrain,
}

impl TownCell {
    pub fn new(terrain: Terrain) -> Self {
        Self { terrain }
    }
}

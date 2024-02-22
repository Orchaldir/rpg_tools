use crate::model::color::Color;
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

    pub fn get_color(&self) -> Color {
        match self.terrain {
            Terrain::Hill { .. } => Color::SaddleBrown,
            Terrain::Mountain { .. } => Color::Gray,
            Terrain::Plain => Color::Green,
            Terrain::River { .. } => Color::Blue,
        }
    }
}

use crate::model::color::Color;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::terrain::Terrain;
use serde::{Deserialize, Serialize};

/// A tile of the [`town`](crate::model::world::town::Town) map.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TownTile {
    pub terrain: Terrain,
    pub construction: Construction,
}

impl TownTile {
    pub fn new(terrain: Terrain) -> Self {
        Self {
            terrain,
            construction: Construction::None,
        }
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

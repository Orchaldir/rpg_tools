use crate::model::world::building::BuildingId;
use crate::model::world::street::StreetId;
use serde::{Deserialize, Serialize};

/// What is build on a [`tile`](crate::model::world::town::tile::TownTile)
/// of the [`town`](crate::model::world::town::Town)?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Construction {
    Building { id: BuildingId },
    Street { id: StreetId },
    None,
}

impl Construction {
    pub fn is_clear(&self) -> bool {
        self == &Self::None
    }

    pub fn is_present(&self) -> bool {
        self != &Self::None
    }
}

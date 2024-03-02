use crate::model::world::building::BuildingId;
use crate::model::world::street::StreetId;
use serde::{Deserialize, Serialize};

/// What is build on a [`tile`](crate::model::world::town::tile::TownTile)
/// of the [`town`](crate::model::world::town::Town)?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Construction {
    Building { id: BuildingId },
    Street { id: StreetId },
    None,
}

impl Construction {
    /// Is the [`tile`](crate::model::world::town::tile::TownTile) clear of any construction?
    pub fn is_clear(&self) -> bool {
        self == &Self::None
    }

    /// Does the [`tile`](crate::model::world::town::tile::TownTile) have any construction?
    pub fn is_present(&self) -> bool {
        self != &Self::None
    }

    /// Does the [`tile`](crate::model::world::town::tile::TownTile) have any street?
    pub fn is_any_street(&self) -> bool {
        matches!(self, Construction::Street { .. })
    }

    /// Does the [`tile`](crate::model::world::town::tile::TownTile) have a specific street?
    pub fn is_street(&self, street_id: StreetId) -> bool {
        if let Construction::Street { id } = self {
            return id.eq(&street_id);
        }

        false
    }
}

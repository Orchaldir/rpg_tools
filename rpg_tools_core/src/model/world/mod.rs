use crate::model::world::building::{Building, BuildingId};
use crate::model::world::mountain::{Mountain, MountainId};
use crate::model::world::river::{River, RiverId};
use crate::model::world::street::{Street, StreetId};
use crate::model::world::town::{Town, TownId};
use crate::utils::storage::Storage;

pub mod building;
pub mod mountain;
pub mod river;
pub mod street;
pub mod town;

/// Contains the terrain features & settlements.
#[derive(Debug, Default)]
pub struct WorldData {
    pub building_manager: Storage<BuildingId, Building>,
    pub mountain_manager: Storage<MountainId, Mountain>,
    pub river_manager: Storage<RiverId, River>,
    pub street_manager: Storage<StreetId, Street>,
    pub town_manager: Storage<TownId, Town>,
}

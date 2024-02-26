use crate::model::world::building::{Building, BuildingId};
use crate::model::world::mountain::{Mountain, MountainId};
use crate::model::world::river::{River, RiverId};
use crate::model::world::street::{Street, StreetId};
use crate::model::world::town::{Town, TownId};
use crate::utils::io::{load_storage, save_storage};
use crate::utils::storage::Storage;
use anyhow::Result;

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

impl WorldData {
    pub fn load(setting: &str) -> Result<Self> {
        Ok(Self {
            building_manager: load_storage(setting, "building")?,
            mountain_manager: load_storage(setting, "mountain")?,
            river_manager: load_storage(setting, "river")?,
            street_manager: load_storage(setting, "street")?,
            town_manager: load_storage(setting, "town")?,
        })
    }

    pub fn save(&self, setting: &str) {
        save_storage(&self.building_manager, setting);
        save_storage(&self.mountain_manager, setting);
        save_storage(&self.river_manager, setting);
        save_storage(&self.street_manager, setting);
        save_storage(&self.town_manager, setting);
    }
}

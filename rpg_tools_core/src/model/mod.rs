use crate::model::character::{Character, CharacterId};
use crate::model::world::building::{Building, BuildingId};
use crate::model::world::mountain::{Mountain, MountainId};
use crate::model::world::river::{River, RiverId};
use crate::model::world::street::{Street, StreetId};
use crate::model::world::town::{Town, TownId};
use crate::utils::io::{load_storage, save_storage};
use crate::utils::storage::Storage;
use std::path::PathBuf;

pub mod character;
pub mod color;
pub mod math;
pub mod world;

/// Contains all the data of the rpg.
#[derive(Debug, Default)]
pub struct RpgData {
    pub setting: String,
    pub building_manager: Storage<BuildingId, Building>,
    pub characters: Storage<CharacterId, Character>,
    pub mountain_manager: Storage<MountainId, Mountain>,
    pub river_manager: Storage<RiverId, River>,
    pub street_manager: Storage<StreetId, Street>,
    pub town_manager: Storage<TownId, Town>,
}

impl RpgData {
    pub fn load(setting: &str) -> anyhow::Result<Self> {
        Ok(Self {
            setting: setting.to_string(),
            building_manager: load_storage(setting, "building")?,
            characters: load_storage(setting, "character")?,
            mountain_manager: load_storage(setting, "mountain")?,
            river_manager: load_storage(setting, "river")?,
            street_manager: load_storage(setting, "street")?,
            town_manager: load_storage(setting, "town")?,
        })
    }

    pub fn save(&self) -> anyhow::Result<()> {
        save_storage(&self.building_manager, &self.setting)?;
        save_storage(&self.characters, &self.setting)?;
        save_storage(&self.mountain_manager, &self.setting)?;
        save_storage(&self.river_manager, &self.setting)?;
        save_storage(&self.street_manager, &self.setting)?;
        save_storage(&self.town_manager, &self.setting)
    }
}

pub fn get_setting_path(setting: &str, file: &str) -> PathBuf {
    ["resources", "settings", setting, file].iter().collect()
}

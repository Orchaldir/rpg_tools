use crate::model::world::WorldData;
use std::path::PathBuf;

pub mod color;
pub mod math;
pub mod world;

#[derive(Debug, Default)]
pub struct RpgData {
    pub setting: String,
    pub world: WorldData,
}

impl RpgData {
    pub fn empty(setting: String) -> Self {
        RpgData {
            setting,
            ..RpgData::default()
        }
    }

    pub fn get_path(&self, file: &str) -> PathBuf {
        get_setting_path(&self.setting, file)
    }

    pub fn save(&self) {
        self.world.save(&self.setting);
    }
}

pub fn get_setting_path(setting: &str, file: &str) -> PathBuf {
    ["resources", "settings", setting, file].iter().collect()
}

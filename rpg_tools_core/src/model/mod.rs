use std::path::PathBuf;

pub mod character;
pub mod color;
pub mod math;
pub mod world;

pub fn get_setting_path(setting: &str, file: &str) -> PathBuf {
    ["resources", "settings", setting, file].iter().collect()
}

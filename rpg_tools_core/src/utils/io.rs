use crate::model::get_setting_path;
use crate::utils::storage::{Element, Id, Storage};
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let string = fs::read_to_string(path).context(format!("Failed to load {:?}", path))?;
    let data: T = serde_yaml::from_str(&string).context(format!("Failed to parse {:?}", path))?;

    Ok(data)
}

pub fn write<T: Serialize>(object: &T, path: &PathBuf) -> Result<()> {
    let directory = path
        .parent()
        .context(format!("Failed to get directory from {:?}", path))?;
    fs::create_dir_all(directory).context(format!("Failed to create directory {:?}", directory))?;
    let mut file = File::create(path).context(format!("Failed to create {:?}", path))?;
    let s = serde_yaml::to_string(object).context(format!("Failed to serialize {:?}", path))?;

    file.write_all(s.as_bytes())
        .context(format!("Failed to write to {:?}", path))?;

    Ok(())
}

pub fn load_storage<ID: Id + DeserializeOwned, ELEMENT: Element<ID> + DeserializeOwned>(
    setting: &str,
    storage: &str,
) -> Result<Storage<ID, ELEMENT>> {
    let elements: Vec<ELEMENT> = read(&get_path(setting, storage))
        .context(format!("Failed to load to storage {}", storage))?;

    Ok(Storage::new(storage.to_string(), elements))
}

pub fn save_storage<ID: Id + Serialize, ELEMENT: Element<ID> + Serialize>(
    storage: &Storage<ID, ELEMENT>,
    setting: &str,
) -> Result<()> {
    write(storage.get_all(), &get_path(setting, storage.name()))
        .context(format!("Failed to save the {}s", storage.name()))
}

fn get_path(setting: &str, storage: &str) -> PathBuf {
    get_setting_path(setting, &format!("{}.yaml", storage))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::town::{Town, TownId};

    #[test]
    fn create_save_and_load_storage() {
        let name = "element";
        let mut storage: Storage<TownId, Town> = Storage::empty(name);
        storage.create(Town::new);

        save_storage(&storage, "test").unwrap();
        let result = load_storage("test", name).unwrap();

        assert_eq!(result, storage);
    }
}

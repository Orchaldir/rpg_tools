use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let string = fs::read_to_string(path).context(format!("Failed to load {:?}", path))?;
    let data: T = serde_yaml::from_str(&string).context(format!("Failed to parse {:?}", path))?;

    Ok(data)
}

pub fn write<T: Serialize>(object: &T, path: &Path) -> Result<()> {
    let mut file = File::create(path).context(format!("Failed to create {:?}", path))?;
    let s = serde_yaml::to_string(object).context(format!("Failed to serialize {:?}", path))?;

    file.write_all(s.as_bytes())
        .context(format!("Failed to write to {:?}", path))?;

    Ok(())
}

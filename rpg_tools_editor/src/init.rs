use anyhow::{Context, Result};
use rpg_tools_core::model::WorldData;

pub fn init() -> Result<WorldData> {
    WorldData::load("CoC").context("Failed to load world!")
}

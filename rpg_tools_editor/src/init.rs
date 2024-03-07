use anyhow::{Context, Result};
use rpg_tools_core::model::RpgData;

pub fn init() -> Result<RpgData> {
    RpgData::load("CoC").context("Failed to load rpg data!")
}

use crate::model::world::town::{Town, TownId};
use crate::utils::storage::Storage;

pub mod town;

/// Contains the terrain features & settlements.
#[derive(Debug, Default)]
pub struct WorldData {
    pub town_manager: Storage<TownId, Town>,
}

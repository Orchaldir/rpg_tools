use crate::model::world::town::TownId;
use crate::utils::storage::Storage;

pub mod town;

/// Contains the terrain features & settlements.
#[derive(Debug)]
pub struct WorldData {
    pub town_manager: Storage<TownId, TownId>,
}

impl Default for WorldData {
    fn default() -> Self {
        Self {
            town_manager: Default::default(),
        }
    }
}

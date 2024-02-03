use crate::model::world::river::{River, RiverId};
use crate::model::world::town::{Town, TownId};
use crate::utils::storage::Storage;

pub mod river;
pub mod town;

/// Contains the terrain features & settlements.
#[derive(Debug, Default)]
pub struct WorldData {
    pub river_manager: Storage<RiverId, River>,
    pub town_manager: Storage<TownId, Town>,
}

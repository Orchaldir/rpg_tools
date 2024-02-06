use crate::model::world::mountain::{Mountain, MountainId};
use crate::model::world::river::{River, RiverId};
use crate::model::world::town::{Town, TownId};
use crate::utils::storage::Storage;

pub mod mountain;
pub mod river;
pub mod town;

/// Contains the terrain features & settlements.
#[derive(Debug, Default)]
pub struct WorldData {
    pub mountain_manager: Storage<MountainId, Mountain>,
    pub river_manager: Storage<RiverId, River>,
    pub town_manager: Storage<TownId, Town>,
}

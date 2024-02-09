use rpg_tools_core::model::world::mountain::{Mountain, MountainId};
use rpg_tools_core::model::world::river::{River, RiverId};
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::storage::Storage;

pub fn init() -> WorldData {
    let mut mountain_manager: Storage<MountainId, Mountain> = Storage::default();
    let hill_id = mountain_manager.create();
    mountain_manager
        .get_mut(hill_id)
        .unwrap()
        .set_name("Hangman's Hill".to_string());

    let mut river_manager: Storage<RiverId, River> = Storage::default();
    let river_id = river_manager.create();
    river_manager
        .get_mut(river_id)
        .unwrap()
        .set_name("Miskatonic River".to_string());

    let mut town_manager: Storage<TownId, Town> = Storage::default();
    let town_id = town_manager.create();
    town_manager
        .get_mut(town_id)
        .unwrap()
        .set_name("Arkham".to_string());

    WorldData {
        mountain_manager,
        river_manager,
        street_manager: Default::default(),
        town_manager,
    }
}

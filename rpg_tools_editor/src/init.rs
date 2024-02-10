use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::model::world::mountain::{Mountain, MountainId};
use rpg_tools_core::model::world::river::{River, RiverId};
use rpg_tools_core::model::world::street::{Street, StreetId};
use rpg_tools_core::model::world::town::edge::TownEdge;
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::map::edge::EdgeMap;
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

    let mut street_manager: Storage<StreetId, Street> = Storage::default();
    let street_id = street_manager.create();
    street_manager
        .get_mut(street_id)
        .unwrap()
        .set_name("Armitage Street".to_string());

    let mut town_manager: Storage<TownId, Town> = Storage::default();
    let town_id = town_manager.create();
    town_manager
        .get_mut(town_id)
        .unwrap()
        .set_name("Arkham".to_string());
    town_manager.get_mut(town_id).unwrap().map = EdgeMap::simple(
        Size2d::new(18, 20),
        TownTile::new(Terrain::Plain),
        TownEdge::None,
    );

    river_manager
        .get_mut(river_id)
        .unwrap()
        .towns
        .insert(town_id);
    street_manager
        .get_mut(street_id)
        .unwrap()
        .towns
        .insert(town_id);

    WorldData {
        mountain_manager,
        river_manager,
        street_manager,
        town_manager,
    }
}

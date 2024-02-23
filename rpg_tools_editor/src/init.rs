use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::model::world::building::lot::BuildingLot;
use rpg_tools_core::model::world::building::{Building, BuildingId};
use rpg_tools_core::model::world::mountain::{Mountain, MountainId};
use rpg_tools_core::model::world::river::{River, RiverId};
use rpg_tools_core::model::world::street::{Street, StreetId};
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::map::tile::TileMap;
use rpg_tools_core::utils::storage::{Element, Storage};

pub fn init() -> WorldData {
    let mut mountain_manager: Storage<MountainId, Mountain> = Storage::default();
    let hill_id = mountain_manager.create(Mountain::new);
    mountain_manager
        .get_mut(hill_id)
        .unwrap()
        .set_name("Hangman's Hill".to_string());

    let mut river_manager: Storage<RiverId, River> = Storage::default();
    let river_id = river_manager.create(River::new);
    river_manager
        .get_mut(river_id)
        .unwrap()
        .set_name("Miskatonic River".to_string());

    let mut street_manager: Storage<StreetId, Street> = Storage::default();
    let street_id = street_manager.create(Street::new);
    street_manager
        .get_mut(street_id)
        .unwrap()
        .set_name("Armitage Street".to_string());

    let mut town_manager: Storage<TownId, Town> = Storage::default();
    let town_id = town_manager.create(Town::new);
    town_manager
        .get_mut(town_id)
        .unwrap()
        .set_name("Arkham".to_string());
    town_manager.get_mut(town_id).unwrap().map =
        TileMap::simple(Size2d::new(18, 20), TownTile::new(Terrain::Plain));
    let mut building_manager: Storage<BuildingId, Building> = Storage::default();
    let building_id = building_manager.create(|id| {
        Building::new(
            id,
            BuildingLot {
                town: town_id,
                tile: 7,
                size: Size2d::new(2, 3),
            },
        )
    });
    building_manager
        .get_mut(building_id)
        .unwrap()
        .set_name("Orne Library".to_string());

    town_manager
        .get_mut(town_id)
        .unwrap()
        .map
        .get_tile_mut(0)
        .unwrap()
        .terrain = Terrain::Hill { id: hill_id };

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
        building_manager,
        mountain_manager,
        river_manager,
        street_manager,
        town_manager,
    }
}

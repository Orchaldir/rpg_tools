use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::BuildingId;
use crate::model::world::street::StreetId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::TownId;
use crate::model::WorldData;

// Construction

pub fn get_construction(data: &WorldData, town_id: TownId, tile: usize) -> Option<&Construction> {
    data.town_manager
        .get(town_id)
        .and_then(|town| town.map.get_tile(tile))
        .map(|tile| &tile.construction)
}

pub fn get_constructions(data: &WorldData, town_id: TownId) -> Vec<&Construction> {
    data.town_manager
        .get(town_id)
        .iter()
        .flat_map(|town| town.map.get_tiles())
        .map(|a| &a.construction)
        .collect()
}

pub fn check_construction<F: FnOnce(&Construction) -> bool>(
    data: &WorldData,
    town_id: TownId,
    tile: usize,
    check: F,
) -> bool {
    get_construction(data, town_id, tile)
        .map(check)
        .unwrap_or(false)
}

pub fn is_construction(
    data: &WorldData,
    town_id: TownId,
    tile: usize,
    construction: Construction,
) -> bool {
    check_construction(data, town_id, tile, |c| c.eq(&construction))
}

pub fn is_lot_construction(
    data: &WorldData,
    lot: &BuildingLot,
    construction: Construction,
) -> bool {
    data.town_manager
        .get(lot.town)
        .map(|town| town.is_lot_construction(lot, &construction))
        .unwrap_or(false)
}

pub fn is_building(
    data: &WorldData,
    town_id: TownId,
    tile: usize,
    building_id: BuildingId,
) -> bool {
    is_construction(
        data,
        town_id,
        tile,
        Construction::Building { id: building_id },
    )
}

pub fn is_free(data: &WorldData, town_id: TownId, tile: usize) -> bool {
    is_construction(data, town_id, tile, Construction::None)
}

pub fn is_lot_free(data: &WorldData, lot: &BuildingLot) -> bool {
    is_lot_construction(data, lot, Construction::None)
}

pub fn is_street(data: &WorldData, town_id: TownId, tile: usize, street_id: StreetId) -> bool {
    is_construction(data, town_id, tile, Construction::Street { id: street_id })
}

pub fn is_any_street(data: &WorldData, town_id: TownId, tile: usize) -> bool {
    check_construction(data, town_id, tile, |construction| {
        matches!(construction, Construction::Street { .. })
    })
}

// Terrain

pub fn is_terrain(data: &WorldData, town_id: TownId, tile: usize, terrain: &Terrain) -> bool {
    data.town_manager
        .get(town_id)
        .and_then(|town| town.map.get_tile(tile))
        .map(|tile| tile.terrain.eq(terrain))
        .unwrap_or(false)
}

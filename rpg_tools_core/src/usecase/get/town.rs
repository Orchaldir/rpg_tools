use crate::model::world::building::BuildingId;
use crate::model::world::street::StreetId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;

pub fn get_construction(data: &WorldData, town_id: TownId, tile: usize) -> Option<&Construction> {
    data.town_manager
        .get(town_id)
        .and_then(|town| town.map.get_tile(tile))
        .map(|tile| &tile.construction)
}

pub fn is_construction(
    data: &WorldData,
    town_id: TownId,
    tile: usize,
    construction: Construction,
) -> bool {
    get_construction(data, town_id, tile)
        .map(|c| c.eq(&construction))
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

pub fn is_street(data: &WorldData, town_id: TownId, tile: usize, street_id: StreetId) -> bool {
    is_construction(data, town_id, tile, Construction::Street { id: street_id })
}

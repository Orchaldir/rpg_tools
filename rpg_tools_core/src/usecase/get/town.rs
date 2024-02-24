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

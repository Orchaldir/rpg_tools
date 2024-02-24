use crate::model::world::street::StreetId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to a [`street`](Street).
pub fn add_street_to_tile(
    data: &mut WorldData,
    town_id: TownId,
    tile: usize,
    id: StreetId,
) -> Result<()> {
    if let Some(town) = data.town_manager.get_mut(town_id) {
        if let Some(tile) = town.map.get_tile_mut(tile) {
            if tile.construction == Construction::None {
                tile.construction = Construction::Street { id };

                return Ok(());
            }
        } else {
            bail!("Tile {} is outside town {}!", tile, town.name());
        }

        bail!("Tile {} in town {} is occupied!", tile, town.name(),);
    } else {
        bail!("Unknown town id {}!", town_id.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::building::lot::BuildingLot;
    use crate::model::world::street::Street;
    use crate::model::world::town::{Town, TownId};
    use crate::model::world::WorldData;
    use crate::usecase::create::building::create_building;
    use crate::usecase::get::town::{is_building, is_construction, is_street};

    #[test]
    fn create_successful() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());
        assert!(is_street(&data, town_id, 0, street_id));
    }

    #[test]
    fn unknown_town() {
        let mut data = WorldData::default();
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, TownId::new(0), 0, street_id).is_err());
    }

    #[test]
    fn outside_map() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 10, street_id).is_err());
        assert!(is_construction(&data, town_id, 0, Construction::None));
    }

    #[test]
    fn occupied_by_building() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);
        let building_id = create_building(&mut data, BuildingLot::new(town_id, 0)).unwrap();

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_err());
        assert!(is_building(&data, town_id, 0, building_id));
    }

    #[test]
    fn occupied_by_street() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);
        let street_id1 = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());
        assert!(add_street_to_tile(&mut data, town_id, 0, street_id1).is_err());
        assert!(is_street(&data, town_id, 0, street_id));
    }
}

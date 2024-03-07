use crate::model::world::street::StreetId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::towns::WithTowns;
use crate::model::world::town::TownId;
use crate::model::WorldData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to add a [`street`](Street) to a [`tile`](crate::model::world::town::tile::TownTile).
pub fn add_street_to_tile(
    data: &mut WorldData,
    town_id: TownId,
    tile: usize,
    street_id: StreetId,
) -> Result<()> {
    if let Some(town) = data.town_manager.get_mut(town_id) {
        if let Some(tile) = town.map.get_tile_mut(tile) {
            if let Some(street) = data.street_manager.get_mut(street_id) {
                if tile.construction == Construction::None {
                    tile.construction = Construction::Street { id: street_id };
                    street.towns_mut().insert(town_id);

                    return Ok(());
                }
            } else {
                bail!("Unknown street id {}!", street_id.id());
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
    use crate::model::WorldData;
    use crate::usecase::create::building::create_building;
    use crate::usecase::get::town::{is_building, is_free, is_street};
    use crate::usecase::get::towns::contains_town;

    #[test]
    fn create_successful() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());

        assert!(is_street(&data, town_id, 0, street_id));
        assert!(contains_town(&data.street_manager, street_id, town_id));
    }

    #[test]
    fn unknown_street() {
        let mut data = WorldData::default();
        let street_id = StreetId::new(0);
        let town_id = data.town_manager.create(Town::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_err());

        assert!(is_free(&data, town_id, 0));
        assert!(data.street_manager.is_empty());
    }

    #[test]
    fn unknown_town() {
        let mut data = WorldData::default();
        let street_id = data.street_manager.create(Street::new);
        let town_id = TownId::new(0);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_err());

        assert!(!contains_town(&data.street_manager, street_id, town_id));
    }

    #[test]
    fn outside_map() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 10, street_id).is_err());

        assert!(is_free(&data, town_id, 0));
        assert!(!contains_town(&data.street_manager, street_id, town_id));
    }

    #[test]
    fn occupied_by_building() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);
        let building_id = create_building(&mut data, BuildingLot::new(town_id, 0)).unwrap();

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_err());

        assert!(is_building(&data, town_id, 0, building_id));
        assert!(!contains_town(&data.street_manager, street_id, town_id));
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
        assert!(!contains_town(&data.street_manager, street_id1, town_id));
    }
}

use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::{Building, BuildingId};
use crate::model::world::town::construction::Construction;
use crate::model::RpgData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to add a [`building`](Building) to a [`tile`](crate::model::world::town::tile::TownTile).
pub fn create_building(data: &mut RpgData, lot: BuildingLot) -> Result<BuildingId> {
    if let Some(town) = data.town_manager.get_mut(lot.town) {
        if town.is_lot_free(&lot) {
            let id = data
                .building_manager
                .create(|id| Building::new(id, lot.clone()));
            let construction = Construction::Building { id };

            if town.set_lot_construction(&lot, construction) {
                return Ok(id);
            }

            panic!(
                "Created building {}, but couldn't set construction of lot!",
                id.id()
            )
        } else {
            bail!(
                "Lot with tile={} & size={}x{} is outside town {}!",
                lot.tile,
                lot.size.width(),
                lot.size.height(),
                town.name()
            );
        }
    } else {
        bail!("Unknown town id {}!", lot.town.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::math::size2d::Size2d;
    use crate::model::world::street::Street;
    use crate::model::world::town::{Town, TownId};
    use crate::model::RpgData;
    use crate::usecase::edit::resize::resize_town;
    use crate::usecase::edit::town::add_street::add_street_to_tile;
    use crate::usecase::get::town::{is_building, is_street};

    #[test]
    fn create_successful() {
        let mut data = RpgData::default();
        let town_id = data.town_manager.create(Town::new);

        let id = create_building(&mut data, BuildingLot::tile(0)).unwrap();

        assert_first_building(&data, town_id, id, 0);
    }

    #[test]
    fn unknown_town() {
        let mut data = RpgData::default();

        assert!(create_building(&mut data, BuildingLot::tile(0)).is_err());
        assert!(data.building_manager.is_empty())
    }

    #[test]
    fn outside_map() {
        let mut data = RpgData::default();
        data.town_manager.create(Town::new);

        assert!(create_building(&mut data, BuildingLot::tile(1)).is_err());
        assert!(data.building_manager.is_empty())
    }

    #[test]
    fn partly_outside_map() {
        let mut data = RpgData::default();
        let town_id = data.town_manager.create(Town::new);
        let lot = BuildingLot::big(town_id, 0, Size2d::square(2));

        assert!(create_building(&mut data, lot).is_err());
        assert!(data.building_manager.is_empty())
    }

    #[test]
    fn occupied_by_building() {
        let mut data = RpgData::default();
        let town_id = data.town_manager.create(Town::new);
        let id = create_building(&mut data, BuildingLot::tile(0)).unwrap();

        assert!(create_building(&mut data, BuildingLot::tile(0)).is_err());

        assert_first_building(&data, town_id, id, 0);
    }

    #[test]
    fn occupied_by_street() {
        let mut data = RpgData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());
        assert!(create_building(&mut data, BuildingLot::tile(0)).is_err());

        assert!(data.building_manager.is_empty());
        assert!(is_street(&data, town_id, 0, street_id));
    }

    #[test]
    fn partly_occupied() {
        for tile in 0..4 {
            other_building_occupies_tile(tile);
        }
    }

    fn other_building_occupies_tile(tile: usize) {
        let mut data = RpgData::default();
        let town_id = data.town_manager.create(Town::new);
        assert!(resize_town(&mut data, town_id, 3, 2).is_ok());
        let other_id = create_building(&mut data, BuildingLot::tile(tile)).unwrap();
        let lot = BuildingLot::big(town_id, 0, Size2d::new(3, 2));

        assert!(create_building(&mut data, lot).is_err());

        assert_first_building(&data, town_id, other_id, tile);
    }

    fn assert_first_building(data: &RpgData, town_id: TownId, id: BuildingId, tile: usize) {
        assert_eq!(id.id(), 0);
        assert_eq!(
            data.building_manager.get(id).unwrap(),
            &Building::new(id, BuildingLot::tile(tile))
        );
        assert!(is_building(&data, town_id, tile, id));
    }
}

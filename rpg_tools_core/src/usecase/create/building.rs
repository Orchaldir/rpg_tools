use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::{Building, BuildingId};
use crate::model::world::town::construction::Construction;
use crate::model::world::WorldData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to add a [`building`](Building) to a [`tile`](crate::model::world::town::tile::TownTile).
pub fn create_building(data: &mut WorldData, lot: BuildingLot) -> Result<BuildingId> {
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
    use crate::model::world::street::Street;
    use crate::model::world::town::{Town, TownId};
    use crate::model::world::WorldData;
    use crate::usecase::create::street::add_street_to_tile;
    use crate::usecase::get::town::{is_building, is_street};

    #[test]
    fn create_successful() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);

        let id = create_building(&mut data, create_lot(0)).unwrap();

        assert_first_building(&data, town_id, id);
    }

    #[test]
    fn unknown_town() {
        let mut data = WorldData::default();

        assert!(create_building(&mut data, create_lot(0)).is_err());
        assert!(data.building_manager.get_all().is_empty())
    }

    #[test]
    fn outside_map() {
        let mut data = WorldData::default();
        data.town_manager.create(Town::new);

        assert!(create_building(&mut data, create_lot(1)).is_err());
        assert!(data.building_manager.get_all().is_empty())
    }

    #[test]
    fn occupied_by_building() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let id = create_building(&mut data, create_lot(0)).unwrap();

        assert!(create_building(&mut data, create_lot(0)).is_err());

        assert_first_building(&data, town_id, id);
    }

    #[test]
    fn occupied_by_street() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());
        assert!(create_building(&mut data, create_lot(0)).is_err());

        assert!(data.building_manager.get_all().is_empty());
        assert!(is_street(&data, town_id, 0, street_id));
    }

    fn create_lot(tile: usize) -> BuildingLot {
        BuildingLot::new(TownId::default(), tile)
    }

    fn assert_first_building(data: &WorldData, town_id: TownId, id: BuildingId) {
        assert_eq!(id.id(), 0);
        assert_eq!(
            data.building_manager.get(id).unwrap(),
            &Building::new(id, create_lot(0))
        );
        assert!(is_building(&data, town_id, 0, id));
    }
}

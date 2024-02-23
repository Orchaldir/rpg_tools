use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::{Building, BuildingId};
use crate::model::world::town::construction::Construction;
use crate::model::world::WorldData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to a [`building`](Building).
pub fn create_building(data: &mut WorldData, lot: BuildingLot) -> Result<BuildingId> {
    if let Some(town) = data.town_manager.get_mut(lot.town) {
        if let Some(tile) = town.map.get_tile_mut(lot.tile) {
            if tile.construction == Construction::None {
                let id = data.building_manager.create(|id| Building::new(id, lot));
                tile.construction = Construction::Building { id };

                return Ok(id);
            }
        } else {
            bail!("Tile {} is outside town {}!", lot.tile, town.name());
        }

        bail!("Tile {} in town {} is occupied!", lot.tile, town.name(),);
    } else {
        bail!("Unknown town id {}!", lot.town.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::math::size2d::Size2d;
    use crate::model::world::town::{Town, TownId};
    use crate::model::world::WorldData;

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

        assert_first_building(&data, town_id, id);
    }

    fn create_lot(tile: usize) -> BuildingLot {
        BuildingLot {
            town: TownId::default(),
            tile,
            size: Size2d::square(1),
        }
    }

    fn assert_first_building(data: &WorldData, town_id: TownId, id: BuildingId) {
        assert_eq!(id.id(), 0);
        assert_eq!(
            data.building_manager.get(id).unwrap(),
            &Building::new(id, create_lot(0))
        );
        assert_eq!(
            data.town_manager
                .get(town_id)
                .unwrap()
                .map
                .get_tile(0)
                .unwrap()
                .construction,
            Construction::Building { id }
        );
    }
}

use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::{Building, BuildingId};
use crate::model::world::WorldData;
use crate::utils::storage::Id;
use anyhow::{bail, Result};

/// Tries to a [`building`](Building).
pub fn create_building(data: &mut WorldData, lot: BuildingLot) -> Result<BuildingId> {
    if let Some(town) = data.town_manager.get(lot.town) {
        if let Some(tile) = town.map.get_tile(lot.tile) {
            return Ok(data.building_manager.create(|id| Building::new(id, lot)));
        }

        bail!("Tile {} is outside town id {}!", lot.tile, lot.town.id());
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
        data.town_manager.create(Town::new);

        let id = create_building(&mut data, create_lot(0)).unwrap();

        assert_eq!(id.id(), 0);
        assert_eq!(
            data.building_manager.get(id).unwrap(),
            &Building::new(id, create_lot(0))
        )
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

    fn create_lot(tile: usize) -> BuildingLot {
        BuildingLot {
            town: TownId::default(),
            tile,
            size: Size2d::square(1),
        }
    }
}

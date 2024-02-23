use crate::model::math::size2d::Size2d;
use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::{Building, BuildingId};
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use crate::utils::storage::Id;
use anyhow::{bail, Result};

/// Tries to update the name of an [`element`](Element).
pub fn create_building(data: &mut WorldData, lot: BuildingLot) -> Result<BuildingId> {
    if data.town_manager.get(lot.town).is_none() {
        bail!("Unknown town id {}!", lot.town.id());
    }

    Ok(data.building_manager.create(|id| Building::new(id, lot)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::town::Town;
    use crate::model::world::WorldData;

    #[test]
    fn create_successful() {
        let mut data = WorldData::default();
        data.town_manager.create(Town::new);

        assert_eq!(
            create_building(
                &mut data,
                BuildingLot {
                    town: TownId::default(),
                    tile: 7,
                    size: Size2d::new(2, 3),
                }
            )
            .unwrap(),
            BuildingId::default()
        );
    }

    #[test]
    fn unknown_town() {
        let mut data = WorldData::default();

        assert!(create_building(
            &mut data,
            BuildingLot {
                town: TownId::default(),
                tile: 7,
                size: Size2d::new(2, 3),
            }
        )
        .is_err());
    }
}

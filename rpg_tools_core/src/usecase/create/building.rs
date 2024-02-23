use crate::model::math::size2d::Size2d;
use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::{Building, BuildingId};
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use anyhow::Result;

/// Tries to update the name of an [`element`](Element).
pub fn create_building(data: &mut WorldData, lot: BuildingLot) -> Result<BuildingId> {
    Ok(data.building_manager.create(|id| Building::new(id, lot)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::WorldData;

    #[test]
    fn create_successful() {
        let mut data = WorldData::default();

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
}

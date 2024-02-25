use crate::model::math::size2d::Size2d;
use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::BuildingId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::tile::TownTile;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use crate::utils::storage::Id;
use anyhow::{bail, Context, Result};

pub fn resize_town(data: &mut WorldData, id: TownId, width: u32, height: u32) -> Result<()> {
    data.town_manager
        .get_mut(id)
        .map(|town| {
            let new_map = town
                .map
                .resize(Size2d::new(width, height), TownTile::new(Terrain::Plain));
            town.map = new_map;
        })
        .context("Town doesn't exist")
}

pub fn resize_building(
    data: &mut WorldData,
    building_id: BuildingId,
    width: u32,
    height: u32,
) -> Result<()> {
    let lot = data
        .building_manager
        .get(building_id)
        .map(|building| building.lot().clone())
        .context("Building doesn't exist")?;

    if let Some(town) = data.town_manager.get_mut(lot.town) {
        let new_lot = BuildingLot {
            size: Size2d::new(width, height),
            ..lot
        };
        if town.can_update_building(&new_lot, building_id) {
            if !town.set_lot_construction(&lot, Construction::None) {
                panic!("Couldn't clear lot")
            }
            if !town.set_lot_construction(&new_lot, Construction::Building { id: building_id }) {
                panic!("Couldn't update lot")
            }

            Ok(())
        } else {
            bail!("");
        }
    } else {
        bail!("Town doesn't exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::town::TownId;
    use crate::model::world::WorldData;

    #[test]
    fn resize_non_existing_town() {
        let mut data = WorldData::default();

        assert!(resize_town(&mut data, TownId::default(), 2, 3).is_err());
        assert!(data.town_manager.get_all().is_empty());
    }
}

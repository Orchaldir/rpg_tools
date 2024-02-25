use crate::model::math::size2d::Size2d;
use crate::model::world::building::lot::BuildingLot;
use crate::model::world::building::BuildingId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::tile::TownTile;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
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

            data.building_manager.get_mut(building_id).map(|building| {
                building.lot = new_lot.clone();
            });
        } else {
            bail!("");
        }
    } else {
        bail!("Town doesn't exist")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::building::Building;
    use crate::model::world::town::{Town, TownId};
    use crate::model::world::WorldData;
    use crate::usecase::create::building::create_building;
    use crate::usecase::get::town::get_constructions;

    #[test]
    fn resize_non_existing_town() {
        let mut data = WorldData::default();

        assert!(resize_town(&mut data, TownId::default(), 2, 3).is_err());

        assert!(data.town_manager.get_all().is_empty());
    }

    #[test]
    fn resize_building_successful() {
        let mut data = WorldData::default();
        let town_id = data
            .town_manager
            .create(|id| Town::simple(id, Size2d::new(3, 2)));
        let old_lot = BuildingLot::new(town_id, 0);
        let new_lot = BuildingLot::big(town_id, 0, Size2d::square(2));
        let building_id = create_building(&mut data, old_lot).unwrap();
        let construction = Construction::Building { id: building_id };

        assert!(resize_building(&mut data, building_id, 2, 2).is_ok());

        assert_eq!(
            data.building_manager.get_all(),
            &vec![Building::new(building_id, new_lot)]
        );
        assert_eq!(
            get_constructions(&data, town_id),
            vec![
                &construction,
                &construction,
                &Construction::None,
                &construction,
                &construction,
                &Construction::None
            ]
        )
    }

    #[test]
    fn resize_building_with_unknown_town() {
        let mut data = WorldData::default();

        assert!(resize_building(&mut data, BuildingId::default(), 2, 2).is_err());
        assert!(data.town_manager.get_all().is_empty());
        assert!(data.building_manager.get_all().is_empty());
    }

    #[test]
    fn resize_building_with_unknown_building() {
        let mut data = WorldData::default();
        data.town_manager
            .create(|id| Town::simple(id, Size2d::new(3, 2)));

        assert!(resize_building(&mut data, BuildingId::default(), 2, 2).is_err());
    }

    #[test]
    fn resize_building_with_too_small_map() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let lot = BuildingLot::new(town_id, 0);
        let building_id = create_building(&mut data, lot.clone()).unwrap();
        let construction = Construction::Building { id: building_id };

        assert!(resize_building(&mut data, building_id, 2, 2).is_err());

        assert_eq!(
            data.building_manager.get_all(),
            &vec![Building::new(building_id, lot)]
        );
        assert_eq!(get_constructions(&data, town_id), vec![&construction,])
    }

    #[test]
    fn resize_building_blocked_by_other_building() {
        let mut data = WorldData::default();
        let town_id = data
            .town_manager
            .create(|id| Town::simple(id, Size2d::new(2, 1)));
        let lot = BuildingLot::new(town_id, 0);
        let other_lot = BuildingLot::new(town_id, 1);
        let building_id = create_building(&mut data, lot.clone()).unwrap();
        let other_id = create_building(&mut data, other_lot.clone()).unwrap();

        assert!(resize_building(&mut data, building_id, 2, 2).is_err());

        assert_eq!(
            data.building_manager.get_all(),
            &vec![
                Building::new(building_id, lot),
                Building::new(other_id, other_lot)
            ]
        );
        assert_eq!(
            get_constructions(&data, town_id),
            vec![
                &Construction::Building { id: building_id },
                &Construction::Building { id: other_id },
            ]
        )
    }
}

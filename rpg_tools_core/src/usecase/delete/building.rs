use crate::model::world::building::BuildingId;
use crate::model::world::town::construction::Construction;
use crate::model::WorldData;
use crate::usecase::delete::DeleteResult;
use crate::utils::storage::DeleteElementResult;

/// Tries to delete a [`building`](Building).
pub fn delete_building(data: &mut WorldData, id: BuildingId) -> DeleteResult {
    let building = match data.building_manager.delete(id) {
        DeleteElementResult::SwappedAndRemoved { element, .. } => {
            if let Some(building) = data.building_manager.get_mut(id) {
                if let Some(town) = data.town_manager.get_mut(building.lot.town) {
                    town.set_lot_construction(&building.lot, Construction::None);
                    town.set_lot_construction(&building.lot, Construction::Building { id });
                }
            }

            element
        }
        DeleteElementResult::DeletedLastElement { element } => element,
        DeleteElementResult::NotFound => return DeleteResult::NotFound,
    };

    if let Some(town) = data.town_manager.get_mut(building.lot.town) {
        town.set_lot_construction(&building.lot, Construction::None);
    }

    DeleteResult::Ok
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::math::size2d::Size2d;
    use crate::model::world::building::lot::BuildingLot;
    use crate::model::world::town::Town;
    use crate::model::WorldData;
    use crate::usecase::create::building::create_building;
    use crate::usecase::get::town::{is_building, is_free};

    #[test]
    fn test_swapped_and_removed() {
        let mut data = WorldData::default();
        let town_id = data
            .town_manager
            .create(|id| Town::simple(id, Size2d::square(2)));
        let id0 = create_building(&mut data, BuildingLot::tile(0)).unwrap();
        let id1 = create_building(&mut data, BuildingLot::tile(1)).unwrap();

        assert_eq!(DeleteResult::Ok, delete_building(&mut data, id0));

        assert!(data.building_manager.contains(id0));
        assert!(!data.building_manager.contains(id1));
        assert!(is_free(&data, town_id, 0));
        assert!(is_building(&data, town_id, 1, id0));
    }

    #[test]
    fn test_delete_last() {
        let mut data = WorldData::default();
        let town_id = data
            .town_manager
            .create(|id| Town::simple(id, Size2d::square(2)));
        let id0 = create_building(&mut data, BuildingLot::tile(0)).unwrap();
        let id1 = create_building(&mut data, BuildingLot::tile(1)).unwrap();

        assert_eq!(DeleteResult::Ok, delete_building(&mut data, id1));

        assert!(data.building_manager.contains(id0));
        assert!(!data.building_manager.contains(id1));
        assert!(is_building(&data, town_id, 0, id0));
        assert!(is_free(&data, town_id, 1));
    }

    #[test]
    fn test_not_found() {
        let mut data = WorldData::default();
        let id = BuildingId::default();

        assert_eq!(DeleteResult::NotFound, delete_building(&mut data, id));
    }
}

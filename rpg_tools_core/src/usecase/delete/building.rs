use crate::model::world::building::BuildingId;
use crate::model::world::town::construction::Construction;
use crate::model::world::WorldData;
use crate::usecase::delete::DeleteResult;
use crate::utils::storage::DeleteElementResult;

/// Tries to delete a [`building`](Building).
pub fn delete_building(data: &mut WorldData, id: BuildingId) -> DeleteResult {
    let building = match data.building_manager.delete(id) {
        DeleteElementResult::SwappedAndRemoved {
            element,
            id_to_update,
        } => {
            if let Some(building) = data.building_manager.get_mut(id_to_update) {
                if let Some(town) = data.town_manager.get_mut(building.lot.town) {
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

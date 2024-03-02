use crate::model::world::town::construction::Construction;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to remove a [`street`](Street) from a [`tile`](crate::model::world::town::tile::TownTile).
pub fn remove_street_from_tile(data: &mut WorldData, town_id: TownId, tile: usize) -> Result<()> {
    if let Some(town) = data.town_manager.get_mut(town_id) {
        if let Some(tile) = town.map.get_tile_mut(tile) {
            if tile.construction.is_any_street() {
                tile.construction = Construction::None;

                return Ok(());
            }
        } else {
            bail!("Tile {} is outside town {}!", tile, town.name());
        }

        bail!("Tile {} in town {} is not a street!", tile, town.name(),);
    } else {
        bail!("Unknown town id {}!", town_id.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::street::Street;
    use crate::model::world::town::{Town, TownId};
    use crate::model::world::WorldData;
    use crate::usecase::edit::town::add_street::add_street_to_tile;
    use crate::usecase::get::town::is_any_street;

    #[test]
    fn create_successful() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);
        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());

        assert!(remove_street_from_tile(&mut data, town_id, 0).is_ok());

        assert!(!is_any_street(&data, town_id, 0));
    }

    #[test]
    fn unknown_town() {
        let mut data = WorldData::default();
        data.street_manager.create(Street::new);

        assert!(remove_street_from_tile(&mut data, TownId::new(0), 0).is_err());
    }

    #[test]
    fn outside_map() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        data.street_manager.create(Street::new);

        assert!(remove_street_from_tile(&mut data, town_id, 10).is_err());
    }

    #[test]
    fn no_street() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        data.street_manager.create(Street::new);

        assert!(remove_street_from_tile(&mut data, town_id, 0).is_err());
    }
}

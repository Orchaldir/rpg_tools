use crate::model::world::street::StreetId;
use crate::model::world::town::construction::Construction;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to a [`street`](Street).
pub fn add_street_to_tile(
    data: &mut WorldData,
    town_id: TownId,
    tile: usize,
    id: StreetId,
) -> Result<()> {
    if let Some(town) = data.town_manager.get_mut(town_id) {
        if let Some(tile) = town.map.get_tile_mut(tile) {
            if tile.construction == Construction::None {
                tile.construction = Construction::Street { id };

                return Ok(());
            }
        } else {
            bail!("Tile {} is outside town {}!", tile, town.name());
        }

        bail!("Tile {} in town {} is occupied!", tile, town.name(),);
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

    #[test]
    fn create_successful() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());
        assert_eq!(
            get_construction(&data, town_id, 0).unwrap(),
            &Construction::Street { id: street_id }
        );
    }

    #[test]
    fn unknown_town() {
        let mut data = WorldData::default();
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, TownId::new(0), 0, street_id).is_err());
    }

    #[test]
    fn outside_map() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 10, street_id).is_err());
    }

    #[test]
    fn occupied_by_street() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let street_id = data.street_manager.create(Street::new);

        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_ok());
        assert!(add_street_to_tile(&mut data, town_id, 0, street_id).is_err());
    }

    fn get_construction(data: &WorldData, town_id: TownId, tile: usize) -> Option<&Construction> {
        data.town_manager
            .get(town_id)
            .unwrap()
            .map
            .get_tile(tile)
            .map(|tile| &tile.construction)
    }
}

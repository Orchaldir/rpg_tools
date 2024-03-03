use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use crate::utils::storage::{Element, Id};
use anyhow::{bail, Result};

/// Tries to edit the [`terrain`](Terrain) of a [`tile`](crate::model::world::town::tile::TownTile).
pub fn edit_terrain(
    data: &mut WorldData,
    town_id: TownId,
    tile: usize,
    terrain: Terrain,
) -> Result<()> {
    if let Some(town) = data.town_manager.get_mut(town_id) {
        if let Some(tile) = town.map.get_tile_mut(tile) {
            tile.terrain = terrain;

            Ok(())
        } else {
            bail!("Tile {} is outside town {}!", tile, town.name());
        }
    } else {
        bail!("Unknown town id {}!", town_id.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::mountain::Mountain;
    use crate::model::world::town::Town;
    use crate::usecase::get::town::is_terrain;

    #[test]
    fn success() {
        let mut data = WorldData::default();
        let town_id = data.town_manager.create(Town::new);
        let id = data.mountain_manager.create(Mountain::new);
        let terrain = Terrain::Hill { id };

        assert!(edit_terrain(&mut data, town_id, 0, terrain).is_ok());

        assert!(is_terrain(&data, town_id, 0, &terrain));
    }
}

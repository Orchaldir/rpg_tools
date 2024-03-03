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
}

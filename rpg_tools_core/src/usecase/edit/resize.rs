use crate::model::math::size2d::Size2d;
use crate::model::world::town::edge::TownEdge;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::tile::TownTile;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use anyhow::{Context, Result};

pub fn resize_town(data: &mut WorldData, id: TownId, width: u32, height: u32) -> Result<()> {
    data.town_manager
        .get_mut(id)
        .map(|town| {
            let new_map = town.map.resize(
                Size2d::new(width, height),
                TownTile::new(Terrain::Plain),
                TownEdge::None,
            );
            town.map = new_map;
        })
        .context("Town doesn't exist")
}

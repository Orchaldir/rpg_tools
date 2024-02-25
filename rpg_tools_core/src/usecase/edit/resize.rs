use crate::model::math::size2d::Size2d;
use crate::model::world::town::terrain::Terrain;
use crate::model::world::town::tile::TownTile;
use crate::model::world::town::TownId;
use crate::model::world::WorldData;
use anyhow::{Context, Result};

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

use crate::model::math::size2d::Size2d;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// The edge map is a 2d grid of tiles with edges around each.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TileMap<Tile> {
    /// The size of the tile map.
    size: Size2d,
    /// The tile map.
    tiles: Vec<Tile>,
}

impl<Tile: Clone> TileMap<Tile> {
    /// Creates an edge map with the default tile & edge.
    pub fn simple(size: Size2d, tile: Tile) -> TileMap<Tile> {
        let tiles = vec![tile; size.tiles()];

        Self::new(size, tiles).unwrap()
    }

    /// Creates an edge map.
    pub fn new(size: Size2d, tiles: Vec<Tile>) -> Result<TileMap<Tile>> {
        if size.tiles() != tiles.len() {
            bail!("Tiles don't match size")
        }

        Ok(TileMap { size, tiles })
    }

    /// Resizes an edge map.
    pub fn resize(&self, size: Size2d, tile: Tile) -> TileMap<Tile> {
        let tiles = resize(&self.size, &self.tiles, &size, tile);

        Self::new(size, tiles).unwrap()
    }

    pub fn get_size(&self) -> Size2d {
        self.size
    }

    // Tiles

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, index: usize) -> Option<&Tile> {
        self.tiles.get(index)
    }

    pub fn get_tile_mut(&mut self, index: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(index)
    }
}

fn resize<T: Clone>(size: &Size2d, tiles: &[T], new_size: &Size2d, default: T) -> Vec<T> {
    let mut new_tiles = vec![];

    for y in 0..new_size.height() {
        for x in 0..new_size.width() {
            if x < size.width() && y < size.height() {
                if let Some(index) = size.to_index(x, y) {
                    new_tiles.push(tiles[index].clone());
                }
            } else {
                new_tiles.push(default.clone());
            }
        }
    }

    new_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let size = Size2d::new(2, 3);
        let map = TileMap::simple(size, 2);

        assert_eq!(map.get_size(), size);
        assert_eq!(map.get_tiles(), &vec![2; 6]);
    }

    #[test]
    fn test_resize_to_larger() {
        let old_size = Size2d::new(2, 1);
        let new_size = Size2d::new(3, 2);
        let old_map = TileMap::new(old_size, vec![10, 30]).unwrap();
        let new_map = TileMap::new(new_size, vec![10, 30, 0, 0, 0, 0]).unwrap();

        assert_eq!(new_map, old_map.resize(new_size, 0));
    }

    #[test]
    fn test_resize_to_smaller() {
        let old_size = Size2d::new(3, 2);
        let new_size = Size2d::new(2, 1);
        let old_map = TileMap::new(old_size, vec![10, 20, 30, 40, 50, 60]).unwrap();
        let new_map = TileMap::new(new_size, vec![10, 20]).unwrap();

        assert_eq!(new_map, old_map.resize(new_size, 0));
    }

    #[test]
    fn test_get_invalid_tile() {
        let size = Size2d::new(2, 3);
        let mut map = TileMap::simple(size, 0);

        assert_eq!(map.get_tile(6), None);
        assert_eq!(map.get_tile_mut(6), None);
    }

    #[test]
    fn test_tiles() {
        let size = Size2d::new(2, 3);
        let mut map = TileMap::simple(size, 0);

        *map.get_tile_mut(0).unwrap() = 1;
        *map.get_tile_mut(2).unwrap() = 3;
        *map.get_tile_mut(4).unwrap() = 4;

        assert_eq!(map.get_size(), size);
        assert_eq!(map.get_tiles(), &vec![1, 0, 3, 0, 4, 0]);
    }
}

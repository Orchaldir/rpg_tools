use crate::model::math::side2d::Side2d;
use crate::model::math::size2d::Size2d;

/// The edge map is a 2d grid of tiles with edges around each.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeMap<Tile, Edge> {
    /// The size of the tile map.
    size: Size2d,
    /// The tile map.
    tiles: Vec<Tile>,
    /// The edges at the top & bottom of each tile.
    horizontal_edges: Vec<Edge>,
    /// The edges to the left & right of each tile.
    vertical_edges: Vec<Edge>,
}

impl<Tile: Clone, Edge: Clone> EdgeMap<Tile, Edge> {
    /// Creates an edge map with the default tile & edge.
    pub fn simple(size: Size2d, tile: Tile, edge: Edge) -> EdgeMap<Tile, Edge> {
        let tiles = vec![tile; size.len()];
        let horizontal_edges = vec![edge.clone(); get_horizontal_edges_size(size).len()];
        let vertical_edges = vec![edge; get_vertical_edges_size(size).len()];

        Self::new(size, tiles, horizontal_edges, vertical_edges).unwrap()
    }

    /// Creates am edge map.
    pub fn new(
        size: Size2d,
        tiles: Vec<Tile>,
        horizontal_edges: Vec<Edge>,
        vertical_edges: Vec<Edge>,
    ) -> Option<EdgeMap<Tile, Edge>> {
        if size.len() != tiles.len()
            || get_horizontal_edges_size(size).len() != horizontal_edges.len()
            || get_vertical_edges_size(size).len() != vertical_edges.len()
        {
            return None;
        }

        Some(EdgeMap {
            size,
            tiles,
            horizontal_edges,
            vertical_edges,
        })
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

    /// edges

    pub fn get_horizontal_edges(&self) -> &Vec<Edge> {
        &self.horizontal_edges
    }

    pub fn get_vertical_edges(&self) -> &Vec<Edge> {
        &self.vertical_edges
    }

    pub fn get_edge(&self, tile_index: usize, side: Side2d) -> Option<&Edge> {
        if tile_index >= self.size.len() {
            return None;
        }

        match side {
            Side2d::Top => self.horizontal_edges.get(tile_index),
            Side2d::Left => self.vertical_edges.get(left_of_tile(self.size, tile_index)),
            Side2d::Bottom => self.horizontal_edges.get(below_tile(self.size, tile_index)),
            Side2d::Right => self
                .vertical_edges
                .get(right_of_tile(self.size, tile_index)),
        }
    }

    pub fn get_edge_mut(&mut self, tile_index: usize, side: Side2d) -> Option<&mut Edge> {
        if tile_index >= self.size.len() {
            return None;
        }

        match side {
            Side2d::Top => self.horizontal_edges.get_mut(tile_index),
            Side2d::Left => self
                .vertical_edges
                .get_mut(left_of_tile(self.size, tile_index)),
            Side2d::Bottom => self
                .horizontal_edges
                .get_mut(below_tile(self.size, tile_index)),
            Side2d::Right => self
                .vertical_edges
                .get_mut(right_of_tile(self.size, tile_index)),
        }
    }
}

/// Returns the size of the horizontal edges based on the size of the map.
pub fn get_horizontal_edges_size(size: Size2d) -> Size2d {
    Size2d::new(size.width(), size.height() + 1)
}

/// Returns the size of the vertical edges based on the size of the map.
pub fn get_vertical_edges_size(size: Size2d) -> Size2d {
    Size2d::new(size.width() + 1, size.height())
}

/// Returns the index of the vertical edge to the left of the tile.
pub fn left_of_tile(size: Size2d, tile_index: usize) -> usize {
    tile_index + size.to_y(tile_index) as usize
}

/// Returns the index of the horizontal edge below the tile.
pub fn below_tile(size: Size2d, tile_index: usize) -> usize {
    tile_index + size.width() as usize
}

/// Returns the index of the vertical edge to the right of the tile.
pub fn right_of_tile(size: Size2d, tile_index: usize) -> usize {
    left_of_tile(size, tile_index) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use Side2d::{Bottom, Left, Right, Top};

    #[test]
    fn test_simple() {
        let size = Size2d::new(2, 3);
        let map = EdgeMap::simple(size, 2, 3);

        assert_eq!(map.get_size(), size);
        assert_eq!(map.get_tiles(), &vec![2; 6]);
        assert_eq!(map.get_horizontal_edges(), &vec![3; 8]);
        assert_eq!(map.get_vertical_edges(), &vec![3; 9]);
    }

    #[test]
    fn test_get_invalid_tile() {
        let size = Size2d::new(2, 3);
        let mut map = EdgeMap::simple(size, 0, 0);

        assert_eq!(map.get_tile(6), None);
        assert_eq!(map.get_tile_mut(6), None);
    }

    #[test]
    fn test_tiles() {
        let size = Size2d::new(2, 3);
        let mut map = EdgeMap::simple(size, 0, 0);

        *map.get_tile_mut(0).unwrap() = 1;
        *map.get_tile_mut(2).unwrap() = 3;
        *map.get_tile_mut(4).unwrap() = 4;

        assert_eq!(map.get_size(), size);
        assert_eq!(map.get_tiles(), &vec![1, 0, 3, 0, 4, 0]);
    }

    #[test]
    fn test_get_edge_with_invalid_tile() {
        let size = Size2d::new(2, 3);
        let mut map = EdgeMap::simple(size, 0, 0);

        assert_eq!(map.get_edge(6, Top), None);
        assert_eq!(map.get_edge_mut(6, Top), None);
    }

    #[test]
    fn test_edges() {
        let size = Size2d::new(3, 3);
        let mut map = EdgeMap::simple(size, 0, 0);

        *map.get_edge_mut(4, Top).unwrap() = 10;
        *map.get_edge_mut(4, Left).unwrap() = 20;
        *map.get_edge_mut(4, Bottom).unwrap() = 30;
        *map.get_edge_mut(4, Right).unwrap() = 40;

        assert_eq!(map.get_edge(4, Top), Some(&10));
        assert_eq!(map.get_edge(1, Bottom), Some(&10));

        assert_eq!(map.get_edge(4, Top), Some(&10));
        assert_eq!(map.get_edge(1, Bottom), Some(&10));

        assert_eq!(map.get_edge(4, Left), Some(&20));
        assert_eq!(map.get_edge(3, Right), Some(&20));

        assert_eq!(map.get_edge(4, Bottom), Some(&30));
        assert_eq!(map.get_edge(7, Top), Some(&30));

        assert_eq!(map.get_edge(4, Right), Some(&40));
        assert_eq!(map.get_edge(5, Left), Some(&40));
    }
}

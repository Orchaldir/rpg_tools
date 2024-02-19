use crate::renderer::style::RenderStyle;
use crate::renderer::Renderer;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::utils::map::edge::{
    get_horizontal_edges_size, get_vertical_edges_size, EdgeMap,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeMapRenderer {
    pub tile_size: u32,
    pub edge_size: u32,
    pub border_size: u32,
}

impl EdgeMapRenderer {
    pub fn new(tile_size: u32, edge_size: u32, border_size: u32) -> Self {
        Self {
            tile_size,
            edge_size,
            border_size,
        }
    }

    pub fn calculate_size<Tile: Clone, Edge: Clone>(&self, map: &EdgeMap<Tile, Edge>) -> Size2d {
        map.get_size() * self.tile_size as f32
    }

    pub fn render_tiles<Tile: Clone, Edge: Clone, F: Fn(&Tile) -> Color>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        lookup: F,
    ) {
        let size = map.get_size();
        let tile_size = Size2d::square(self.tile_size);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(tile) = map.get_tile(index) {
                    let position = self.calculate_position(start, x, y);
                    let color = lookup(tile);
                    let style = RenderStyle::with_border(color, Color::Black, self.border_size);
                    renderer.render_rectangle(&AABB::new(position, tile_size), &style);
                }

                index += 1;
            }
        }
    }

    pub fn render_tiles_with_link<
        Tile: Clone,
        Edge: Clone,
        C: Fn(&Tile) -> Color,
        L: Fn(usize, &Tile) -> String,
    >(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        color_lookup: C,
        link_lookup: L,
    ) {
        let size = map.get_size();
        let tile_size = Size2d::square(self.tile_size);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(tile) = map.get_tile(index) {
                    let position = self.calculate_position(start, x, y);
                    let color = color_lookup(tile);
                    let link = link_lookup(index, tile);
                    let style = RenderStyle::with_border(color, Color::Black, self.border_size);

                    renderer.link(&link);
                    renderer.render_rectangle(&AABB::new(position, tile_size), &style);
                    renderer.close();
                }

                index += 1;
            }
        }
    }

    pub fn render_edges<Tile: Clone, Edge: Clone, F: Fn(&Edge) -> Option<Color>>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        lookup: F,
    ) {
        self.render_horizontal_edges(renderer, start, map, &lookup);
        self.render_vertical_edges(renderer, start, map, &lookup);
    }

    fn render_horizontal_edges<Tile: Clone, Edge: Clone, F: Fn(&Edge) -> Option<Color>>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        lookup: F,
    ) {
        let size = get_horizontal_edges_size(map.get_size());
        let edge_size = Size2d::new(self.tile_size + self.edge_size, self.edge_size);
        let edges = map.get_horizontal_edges();

        self.render_internal_edges::<Edge, F>(renderer, start, lookup, size, edge_size, edges);
    }

    fn render_vertical_edges<Tile: Clone, Edge: Clone, F: Fn(&Edge) -> Option<Color>>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        lookup: F,
    ) {
        let size = get_vertical_edges_size(map.get_size());
        let edge_size = Size2d::new(self.edge_size, self.tile_size + self.edge_size);
        let edges = map.get_vertical_edges();

        self.render_internal_edges::<Edge, F>(renderer, start, lookup, size, edge_size, edges);
    }

    fn render_internal_edges<Edge: Clone, F: Fn(&Edge) -> Option<Color>>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        lookup: F,
        size: Size2d,
        edge_size: Size2d,
        edges: &[Edge],
    ) {
        let half = -(self.edge_size as i32 / 2);
        let offset = Point2d::new(half, half);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(edge) = edges.get(index) {
                    if let Some(color) = lookup(edge) {
                        let position = self.calculate_position(start, x, y) + offset;
                        let style = RenderStyle::with_border(color, Color::Black, self.border_size);
                        renderer.render_rectangle(&AABB::new(position, edge_size), &style);
                    }
                }

                index += 1;
            }
        }
    }

    fn calculate_position(&self, start: &Point2d, x: u32, y: u32) -> Point2d {
        *start + Point2d::new((x * self.tile_size) as i32, (y * self.tile_size) as i32)
    }
}

use crate::renderer::style::RenderStyle;
use crate::renderer::Renderer;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::utils::map::edge::{get_horizontal_edges_size, EdgeMap};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeMapRenderer {
    pub cell_size: u32,
    pub edge_size: u32,
    pub border_size: u32,
}

impl EdgeMapRenderer {
    pub fn new(cell_size: u32, edge_size: u32, border_size: u32) -> Self {
        Self {
            cell_size,
            edge_size,
            border_size,
        }
    }

    pub fn calculate_size<Tile: Clone, Edge: Clone>(&self, map: &EdgeMap<Tile, Edge>) -> Size2d {
        map.get_size() * self.cell_size as f32
    }

    pub fn render_tiles<Tile: Clone, Edge: Clone, F: Fn(&Tile) -> Color>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        lookup: F,
    ) {
        let size = map.get_size();
        let tile_size = Size2d::square(self.cell_size);
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

    pub fn render_edges<Tile: Clone, Edge: Clone, F: Fn(&Edge) -> Color>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        lookup: F,
    ) {
        self.render_horizontal_edges(renderer, start, map, lookup);
    }

    fn render_horizontal_edges<Tile: Clone, Edge: Clone, F: Fn(&Edge) -> Color>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Edge>,
        lookup: F,
    ) {
        let size = get_horizontal_edges_size(map.get_size());
        let edge_size = Size2d::new(self.cell_size, self.edge_size);
        let offset = Point2d::new(0, -(self.edge_size as i32 / 2));
        let edges = map.get_horizontal_edges();
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(edge) = edges.get(index) {
                    let position = self.calculate_position(start, x, y) + offset;
                    let color = lookup(edge);
                    let style = RenderStyle::with_border(color, Color::Black, self.border_size);
                    renderer.render_rectangle(&AABB::new(position, edge_size), &style);
                }

                index += 1;
            }
        }
    }

    fn calculate_position(&self, start: &Point2d, x: u32, y: u32) -> Point2d {
        *start + Point2d::new((x * self.cell_size) as i32, (y * self.cell_size) as i32)
    }
}

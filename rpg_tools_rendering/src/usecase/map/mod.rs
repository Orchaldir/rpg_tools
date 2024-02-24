use crate::renderer::style::RenderStyle;
use crate::renderer::{LinkRenderer, Renderer};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::utils::map::tile::TileMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileMapRenderer {
    pub tile_size: u32,
    pub edge_size: u32,
    pub border_size: u32,
}

impl TileMapRenderer {
    pub fn new(tile_size: u32, edge_size: u32, border_size: u32) -> Self {
        Self {
            tile_size,
            edge_size,
            border_size,
        }
    }

    pub fn calculate_size<Tile: Clone>(&self, map: &TileMap<Tile>) -> Size2d {
        map.get_size() * self.tile_size as f32
    }

    pub fn render<Tile: Clone, F: FnMut(usize, AABB, &Tile)>(
        &self,
        start: &Point2d,
        map: &TileMap<Tile>,
        mut rendering: F,
    ) {
        let size = map.get_size();
        let tile_size = Size2d::square(self.tile_size);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(tile) = map.get_tile(index) {
                    let position = self.calculate_position(start, x, y);
                    rendering(index, AABB::new(position, tile_size), tile);
                }

                index += 1;
            }
        }
    }

    pub fn render_tiles<Tile: Clone, F: Fn(&Tile) -> Color>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &TileMap<Tile>,
        lookup: F,
    ) {
        self.render(start, map, |_index, aabb, tile| {
            let color = lookup(tile);
            let style = RenderStyle::with_border(color, Color::Black, self.border_size);
            renderer.render_rectangle(&aabb, &style);
        });
    }

    pub fn render_tiles_with_link<
        Tile: Clone,
        C: Fn(&Tile) -> Color,
        L: Fn(usize, &Tile) -> Option<String>,
    >(
        &self,
        renderer: &mut dyn LinkRenderer,
        start: &Point2d,
        map: &TileMap<Tile>,
        color_lookup: C,
        link_lookup: L,
    ) {
        self.render(start, map, |index, aabb, tile| {
            let color = color_lookup(tile);
            let style = RenderStyle::with_border(color, Color::Black, self.border_size);

            if let Some(link) = link_lookup(index, tile) {
                renderer.link(&link);
                renderer.render_rectangle(&aabb, &style);
                renderer.close();
            } else {
                renderer.render_rectangle(&aabb, &style);
            }
        });
    }

    fn calculate_position(&self, start: &Point2d, x: u32, y: u32) -> Point2d {
        *start + Point2d::new((x * self.tile_size) as i32, (y * self.tile_size) as i32)
    }
}

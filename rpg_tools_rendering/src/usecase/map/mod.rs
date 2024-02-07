use crate::renderer::style::RenderStyle;
use crate::renderer::Renderer;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::utils::map::edge::EdgeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BorderMapRenderer {
    pub cell_size: u32,
    pub border_size: u32,
}

impl BorderMapRenderer {
    pub fn new(cell_size: u32, border_size: u32) -> Self {
        Self {
            cell_size,
            border_size,
        }
    }

    pub fn calculate_size<Tile: Clone, Border: Clone>(
        &self,
        map: &EdgeMap<Tile, Border>,
    ) -> Size2d {
        map.get_size() * self.cell_size as f32
    }

    pub fn render_tiles<Tile: Clone, Border: Clone, F: Fn(&Tile) -> Color>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &EdgeMap<Tile, Border>,
        lookup: F,
    ) {
        let size = map.get_size();
        let cell_size = Size2d::square(self.cell_size);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(tile) = map.get_tile(index) {
                    let position = *start
                        + Point2d::new((x * self.cell_size) as i32, (y * self.cell_size) as i32);
                    let color = lookup(tile);
                    let style = RenderStyle::with_border(color, Color::Black, self.border_size);
                    renderer.render_rectangle(&AABB::new(position, cell_size), &style);
                }

                index += 1;
            }
        }
    }
}

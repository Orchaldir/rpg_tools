use crate::renderer::style::RenderStyle;
use crate::renderer::Renderer;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::utils::map::border::BorderMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TownRenderer {
    pub cell_size: u32,
    pub border_size: u32,
}

impl TownRenderer {
    pub fn new(cell_size: u32, border_size: u32) -> Self {
        Self {
            cell_size,
            border_size,
        }
    }

    pub fn calculate_size<Tile: Clone, Border: Clone>(
        &self,
        map: &BorderMap<Tile, Border>,
    ) -> Size2d {
        map.get_size() * self.cell_size as f32
    }

    pub fn render<Tile: Clone, Border: Clone>(
        &self,
        renderer: &mut dyn Renderer,
        start: &Point2d,
        map: &BorderMap<Tile, Border>,
    ) {
        let size = map.get_size();
        let cell_size = Size2d::square(self.cell_size);
        let style = RenderStyle::with_border(Color::Blue, Color::Green, self.border_size);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                let position =
                    *start + Point2d::new((x * self.cell_size) as i32, (y * self.cell_size) as i32);
                renderer.render_rectangle(&AABB::new(position, cell_size), &style);
                index += 1;
            }
        }
    }
}

use crate::renderer::style::RenderStyle;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;

pub mod style;
pub mod svg;

pub trait Renderer {
    /// Renders a circle.
    fn render_circle(&mut self, center: &Point2d, radius: u32, style: &RenderStyle);

    /// Renders a circle in an [`axis aligned bounding box`](AABB).
    fn render_circle_in_aabb(&mut self, aabb: &AABB, style: &RenderStyle) {
        self.render_circle(&aabb.center(), aabb.inner_radius(), style)
    }

    /// Renders an axis aligned rectangle.
    fn render_rectangle(&mut self, aabb: &AABB, style: &RenderStyle);
}

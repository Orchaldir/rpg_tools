use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::color::Color::Black;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_rendering::renderer::style::RenderStyle;
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use Color::{Blue, Green};

fn main() {
    println!("A SVG example!");

    let size = Size2d::new(500, 500);
    let mut builder = SvgBuilder::new(size);
    let style = RenderStyle::with_border(Green, Blue, 2);

    builder.render_rectangle(&AABB::with_size(size), &RenderStyle::only_border(Black, 1));
    builder.render_rectangle(&AABB::simple(10, 20, 100, 200), &style);
    builder.render_circle(&Point2d::new(300, 300), 100, &style);

    let svg = builder.finish();

    svg.save("example.svg").unwrap();
}

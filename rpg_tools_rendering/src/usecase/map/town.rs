use crate::renderer::style::RenderStyle;
use crate::renderer::svg::builder::SvgBuilder;
use crate::renderer::Renderer;
use crate::usecase::map::TileMapRenderer;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::town::construction::Construction::Building;
use rpg_tools_core::model::world::town::Town;

pub fn render_buildings(mut builder: &mut SvgBuilder, renderer: &TileMapRenderer, town: &Town) {
    renderer.render(&Point2d::default(), &town.map, |_index, aabb, tile| {
        if let Building { .. } = tile.construction {
            render_building(builder, &aabb);
        }
    });
}

pub fn render_building(mut builder: &mut SvgBuilder, aabb: &AABB) {
    let style = RenderStyle::no_border(Color::Black);
    builder.render_rectangle(&aabb.scale(0.5), &style);
}

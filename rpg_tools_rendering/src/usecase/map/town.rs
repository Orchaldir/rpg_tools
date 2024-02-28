use crate::renderer::style::RenderStyle;
use crate::renderer::svg::builder::SvgBuilder;
use crate::renderer::Renderer;
use crate::usecase::map::TileMapRenderer;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::building::Building;
use rpg_tools_core::model::world::street::StreetId;
use rpg_tools_core::model::world::town::construction::Construction;
use rpg_tools_core::model::world::town::construction::Construction::Street;
use rpg_tools_core::model::world::town::Town;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::storage::Element;

pub fn render_buildings(
    data: &WorldData,
    builder: &mut SvgBuilder,
    renderer: &TileMapRenderer,
    town: &Town,
) {
    data.building_manager
        .get_all()
        .iter()
        .filter(|&building| building.lot().town.eq(&town.id()))
        .for_each(|building| render_building(builder, renderer, town, building));
}

pub fn render_building(
    builder: &mut SvgBuilder,
    renderer: &TileMapRenderer,
    town: &Town,
    building: &Building,
) {
    let style = RenderStyle::no_border(Color::Black);
    let start = renderer.calculate_index_position(
        &Point2d::default(),
        town.map.get_size(),
        building.lot().tile,
    );
    let size = renderer.calculate_size(building.lot().size);
    let aabb = AABB::new(start, size).shrink(renderer.tile_size / 4);

    builder.render_rectangle(&aabb, &style);
}

pub fn render_streets(builder: &mut SvgBuilder, renderer: &TileMapRenderer, town: &Town) {
    renderer.render(
        &Point2d::default(),
        &town.map,
        |_index, _x, _y, aabb, tile| {
            if let Street { .. } = tile.construction {
                render_street(builder, &aabb);
            }
        },
    );
}

pub fn render_streets2<F: FnMut(AABB, StreetId)>(
    renderer: &TileMapRenderer,
    town: &Town,
    mut render: F,
) {
    renderer.render(
        &Point2d::default(),
        &town.map,
        |_index, x, y, aabb, tile| {
            if let Street { id } = tile.construction {
                if town.check_construction_xy(x + 1, y, Construction::is_any_street) {
                    let right_aabb = aabb + Point2d::new(renderer.tile_size as i32 / 2, 0);
                    render(right_aabb, id);
                }
                if town.check_construction_xy(x, y + 1, Construction::is_any_street) {
                    let down_aabb = aabb + Point2d::new(0, renderer.tile_size as i32 / 2);
                    render(down_aabb, id);
                }
                render(aabb, id);
            }
        },
    );
}

pub fn render_street(builder: &mut SvgBuilder, aabb: &AABB) {
    let style = RenderStyle::no_border(Color::Gray);
    builder.render_rectangle(&aabb.scale(0.5), &style);
}

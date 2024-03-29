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
use rpg_tools_core::model::RpgData;
use rpg_tools_core::utils::storage::Element;

pub fn render_buildings(
    data: &RpgData,
    builder: &mut SvgBuilder,
    renderer: &TileMapRenderer,
    town: &Town,
) {
    data.building_manager
        .get_all()
        .iter()
        .filter(|&building| building.lot.town.eq(&town.id()))
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
        building.lot.tile,
    );
    let size = renderer.calculate_size(building.lot.size);
    let aabb = AABB::new(start, size).shrink(renderer.tile_size / 4);

    builder.render_rectangle(&aabb, &style);
}

pub fn render_streets(builder: &mut SvgBuilder, renderer: &TileMapRenderer, town: &Town) {
    render_streets_complex(renderer, town, |aabb, _id, _index| {
        render_street(builder, &aabb);
    });
}

pub fn render_streets_complex<F: FnMut(AABB, StreetId, usize)>(
    renderer: &TileMapRenderer,
    town: &Town,
    mut render: F,
) {
    renderer.render(&Point2d::default(), &town.map, |index, x, y, aabb, tile| {
        if let Street { id } = tile.construction {
            if town.check_construction_xy(x + 1, y, Construction::is_any_street) {
                let right_aabb = aabb + Point2d::new(renderer.tile_size as i32 / 2, 0);
                render(right_aabb, id, index);
            }
            if town.check_construction_xy(x, y + 1, Construction::is_any_street) {
                let down_aabb = aabb + Point2d::new(0, renderer.tile_size as i32 / 2);
                render(down_aabb, id, index);
            }
            render(aabb, id, index);
        }
    });
}

pub fn render_street(builder: &mut SvgBuilder, aabb: &AABB) {
    render_street_color(builder, aabb, Color::Gray);
}

pub fn render_street_color(builder: &mut SvgBuilder, aabb: &AABB, color: Color) {
    let style = RenderStyle::no_border(color);
    builder.render_rectangle(&aabb.scale(0.5), &style);
}

pub fn render_constructs(
    data: &RpgData,
    builder: &mut SvgBuilder,
    renderer: &TileMapRenderer,
    town: &Town,
) {
    render_buildings(data, builder, renderer, town);
    render_streets(builder, renderer, town);
}

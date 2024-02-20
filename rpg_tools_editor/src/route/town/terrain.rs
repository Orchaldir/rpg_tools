use crate::svg::RawSvg;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::EdgeMapRenderer;

#[get("/town/terrain/all/<id>")]
pub fn get_all_terrain(state: &State<EditorData>, id: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data, TownId::new(id))
}

#[get("/town/terrain/all/<id>/map.svg")]
pub fn get_terrain_edit_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_to_svg(&state.town_renderer, town))
}

#[get("/town/terrain/edit/<id>/<index>")]
pub fn edit_terrain(state: &State<EditorData>, id: usize, index: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_template(&data, TownId::new(id), index)
}

fn render_to_svg(renderer: &EdgeMapRenderer, town: &Town) -> RawSvg {
    let size = renderer.calculate_size(&town.map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles_with_link(
        &mut builder,
        &Point2d::default(),
        &town.map,
        TownTile::get_color,
        |index, _tile| format!("../../edit/{}/{}", town.id().id(), index),
    );

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

fn get_all_template(data: &WorldData, id: TownId) -> Option<Template> {
    data.town_manager.get(id).map(|town| {
        Template::render(
            "town/terrain/all",
            context! {
                name: town.name(),
                id: id.id(),
            },
        )
    })
}

fn get_edit_template(data: &WorldData, id: TownId, index: usize) -> Option<Template> {
    data.town_manager.get(id).map(|town| {
        Template::render(
            "town/terrain/edit",
            context! {
                name: town.name(),
                id: id.id(),
                index: index,
                terrains: vec!["Hill", "Mountain", "Plain", "River"],
                terrain: town.map.get_tile(index),
            },
        )
    })
}

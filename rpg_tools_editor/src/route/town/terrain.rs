use crate::route::get_all_template;
use crate::route::town::render_to_svg;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::town::edge::TownEdge;
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::usecase::edit::resize::resize_town;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::EdgeMapRenderer;

#[get("/town/terrain/edit/<id>")]
pub fn edit_terrain(state: &State<EditorData>, id: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_template(&data, TownId::new(id))
}

#[get("/town/terrain/edit/<id>/map.svg")]
pub fn get_terrain_edit_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_to_svg(&state.town_renderer, town))
}

fn get_edit_template(data: &WorldData, id: TownId) -> Option<Template> {
    data.town_manager.get(id).map(|town| {
        Template::render(
            "town/terrain/edit",
            context! {
                name: town.name(),
                id: id.id(),
            },
        )
    })
}

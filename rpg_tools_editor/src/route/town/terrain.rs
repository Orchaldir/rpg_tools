use crate::route::get_all_elements;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::form::Form;
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

#[derive(FromForm, Debug)]
pub struct TileUpdate<'r> {
    terrain: &'r str,
    id: u32,
}

#[post("/town/terrain/update/<id>/<index>", data = "<update>")]
pub fn update_tile(
    state: &State<EditorData>,
    id: usize,
    index: usize,
    update: Form<TileUpdate<'_>>,
) -> Option<Template> {
    println!("Update tile {} of town {} with {:?}", index, id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let town_id = TownId::new(id);

    get_all_template(&data, town_id)
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
    let mountains = get_all_elements(&data.mountain_manager);
    let rivers = get_all_elements(&data.river_manager);

    data.town_manager.get(id).map(|town| {
        Template::render(
            "town/terrain/edit",
            context! {
                name: town.name(),
                id: id.id(),
                index: index,
                mountains: mountains,
                rivers: rivers,
                terrains: vec!["Hill", "Mountain", "Plain", "River"],
                tile: town.map.get_tile(index),
            },
        )
    })
}

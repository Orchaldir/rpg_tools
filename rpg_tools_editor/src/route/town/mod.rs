pub mod building;
pub mod street;
pub mod tile;

use crate::html::create_html;
use crate::route::building::link_building_details;
use crate::route::town::building::link_building_creator;
use crate::route::town::street::link_street_creator;
use crate::route::util::get_all_html;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::building::BuildingId;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::usecase::edit::resize::resize_town;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::renderer::{LinkRenderer, Tooltip};
use rpg_tools_rendering::usecase::map::town::{
    render_building, render_street, render_streets_complex,
};
use rpg_tools_rendering::usecase::map::TileMapRenderer;

#[get("/town/all")]
pub fn get_all_towns(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_html(&data.town_manager, "Towns")
}

pub fn link_all_towns() -> String {
    uri!(get_all_towns()).to_string()
}

#[get("/town/new")]
pub fn add_town(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.town_manager.create(Town::new);

    println!("Create town {}", id.id());

    get_edit_html(&data, id, "")
}

#[get("/town/<id>/details")]
pub fn get_town_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_html(&data, TownId::new(id))
}

pub fn link_town_details(id: TownId) -> String {
    uri!(get_town_details(id = id.id())).to_string()
}

#[get("/town/<id>/edit")]
pub fn edit_town(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_html(&data, TownId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct TownUpdate<'r> {
    name: &'r str,
    width: u32,
    height: u32,
}

#[post("/town/<id>/update", data = "<update>")]
pub fn update_town(
    state: &State<EditorData>,
    id: usize,
    update: Form<TownUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update town {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let town_id = TownId::new(id);

    if let Err(e) = update_name(&mut data.town_manager, town_id, update.name) {
        return get_edit_html(&data, town_id, &e.to_string());
    }
    if let Err(e) = resize_town(&mut data, town_id, update.width, update.height) {
        return get_edit_html(&data, town_id, &e.to_string());
    }

    get_details_html(&data, town_id)
}

#[get("/town/<id>/map.svg")]
pub fn get_town_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_town(&data, &state.town_renderer, town, link_building_details))
}

fn get_details_html(data: &WorldData, id: TownId) -> Option<RawHtml<String>> {
    let buildings = data
        .building_manager
        .get_all()
        .iter()
        .filter(|&building| building.lot().town.eq(&id))
        .count();
    let map_uri = uri!(get_town_map(id.id())).to_string();

    data.town_manager.get(id).map(|town| {
        let builder = create_html()
            .h1(&format!("Town: {}", town.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field_usize("Buildings:", buildings)
            .p(|b| b.link(&format!("/town/{}/edit", id.id()), "Edit"))
            .p(|b| b.link(&format!("/town/{}/tile/all", id.id()), "Edit Terrain"))
            .p(|b| b.link(&link_building_creator(id), "Add Buildings"))
            .p(|b| b.link(&link_street_creator(id), "Edit Streets"))
            .p(|b| b.link(&link_all_towns(), "Back"))
            .h2("Map")
            .center(|b| b.svg(&map_uri, "800"));
        RawHtml(builder.finish())
    })
}

fn render_town<F: FnMut(BuildingId) -> String>(
    data: &WorldData,
    renderer: &TileMapRenderer,
    town: &Town,
    mut get_link: F,
) -> RawSvg {
    let size = renderer.calculate_map_size(&town.map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles(
        &mut builder,
        &Point2d::default(),
        &town.map,
        TownTile::get_color,
    );

    data.building_manager
        .get_all()
        .iter()
        .filter(|&building| building.lot().town.eq(&town.id()))
        .for_each(|building| {
            builder.tooltip(building.name());
            builder.link(&get_link(building.id()));
            render_building(&mut builder, renderer, town, building);
            builder.close();
            builder.clear_tooltip();
        });

    render_streets_complex(renderer, town, |aabb, id, _index| {
        if let Some(street) = data.street_manager.get(id) {
            builder.tooltip(street.name())
        }

        render_street(&mut builder, &aabb);
        builder.clear_tooltip();
    });

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

fn get_edit_html(data: &WorldData, id: TownId, name_error: &str) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_town(id.id())).to_string();

    data.town_manager.get(id).map(|town| {
        let builder = create_html()
            .h1(&format!("Edit Town: {}", town.name()))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input("Name", "name", town.name())
                    .error(name_error)
                    .number_input(
                        "Width",
                        "width",
                        town.map.get_size().width() as usize,
                        1,
                        100,
                    )
                    .number_input(
                        "Height",
                        "height",
                        town.map.get_size().height() as usize,
                        1,
                        100,
                    )
            })
            .p(|b| b.link(&link_town_details(id), "Back"));

        RawHtml(builder.finish())
    })
}

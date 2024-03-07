use crate::html::create_html;
use crate::route::town::link_town_details;
use crate::route::util::get_all_elements;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::street::StreetId;
use rpg_tools_core::model::world::town::construction::Construction;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::WorldData;
use rpg_tools_core::usecase::edit::town::add_street::add_street_to_tile;
use rpg_tools_core::usecase::edit::town::remove_street::remove_street_from_tile;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::renderer::LinkRenderer;
use rpg_tools_rendering::usecase::map::town::{
    render_buildings, render_street, render_street_color, render_streets_complex,
};
use rpg_tools_rendering::usecase::map::TileMapRenderer;

#[get("/town/<id>/street/editor")]
pub fn get_street_editor(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    let tools = state.tools.lock().expect("lock shared data");

    get_street_creator_html(&data, TownId::new(id), tools.selected_street)
}

pub fn link_street_editor(id: TownId) -> String {
    uri!(get_street_editor(id.id())).to_string()
}

#[derive(FromForm, Debug)]
pub struct StreetEditorUpdate {
    street: usize,
}

#[post("/town/<id>/street/update", data = "<update>")]
pub fn update_street_editor(
    state: &State<EditorData>,
    id: usize,
    update: Form<StreetEditorUpdate>,
) -> Option<RawHtml<String>> {
    println!("Update street editor {} with {:?}", id, update);
    let data = state.data.lock().expect("lock shared data");
    let mut tools = state.tools.lock().expect("lock shared data");

    tools.selected_street = StreetId::new(update.street);

    get_street_creator_html(&data, TownId::new(id), tools.selected_street)
}

#[get("/town/<id>/street/editor.svg")]
pub fn get_street_editor_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    let tools = state.tools.lock().expect("lock shared tools");

    data.town_manager.get(TownId::new(id)).map(|town| {
        render_street_editor_map(&data, &state.town_renderer, town, tools.selected_street)
    })
}

#[get("/town/<id>/street/add/<tile>")]
pub fn add_street_to_town(
    state: &State<EditorData>,
    id: usize,
    tile: usize,
) -> Option<RawHtml<String>> {
    let mut data = state.data.lock().expect("lock shared data");
    let tools = state.tools.lock().expect("lock shared data");
    let town_id = TownId::new(id);

    if add_street_to_tile(&mut data, town_id, tile, tools.selected_street).is_ok() {
        println!(
            "Added street {} to tile {} of town {}",
            tools.selected_street.id(),
            tile,
            id
        );
    } else {
        println!("Failed to add a street to tile {} of town {}", tile, id);
    }

    get_street_creator_html(&data, town_id, tools.selected_street)
}

pub fn link_add_street_to_town(id: TownId, tile: usize) -> String {
    uri!(add_street_to_town(id.id(), tile)).to_string()
}

#[get("/town/<id>/street/remove/<tile>")]
pub fn remove_street_from_town(
    state: &State<EditorData>,
    id: usize,
    tile: usize,
) -> Option<RawHtml<String>> {
    let mut data = state.data.lock().expect("lock shared data");
    let tools = state.tools.lock().expect("lock shared data");
    let town_id = TownId::new(id);

    if remove_street_from_tile(&mut data, town_id, tile).is_ok() {
        println!(
            "Removed street {} on tile {} of town {}",
            tools.selected_street.id(),
            tile,
            id
        );
    } else {
        println!("Failed to remove a street on tile {} of town {}", tile, id);
    }

    get_street_creator_html(&data, town_id, tools.selected_street)
}

pub fn link_remove_street_from_town(id: TownId, tile: usize) -> String {
    uri!(remove_street_from_town(id.id(), tile)).to_string()
}

fn get_street_creator_html(
    data: &WorldData,
    id: TownId,
    street_id: StreetId,
) -> Option<RawHtml<String>> {
    let map_uri = uri!(get_street_editor_map(id.id())).to_string();
    let back_uri = link_town_details(id);
    let update_uri = uri!(update_street_editor(id.id())).to_string();
    let streets = get_all_elements(&data.street_manager);

    data.town_manager.get(id).map(|town| {
        let builder = create_html()
            .h1(&format!("Add Streets to Town {}", town.name()))
            .form(&update_uri, |b| {
                b.select_id("Street", "street", &streets, street_id.id())
            })
            .center(|b| b.svg(&map_uri, "800"))
            .p(|b| b.link(&back_uri, "Back"));

        RawHtml(builder.finish())
    })
}

fn render_street_editor_map(
    data: &WorldData,
    renderer: &TileMapRenderer,
    town: &Town,
    selected: StreetId,
) -> RawSvg {
    let size = renderer.calculate_map_size(&town.map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_links(
        &mut builder,
        &Point2d::default(),
        &town.map,
        TownTile::get_color,
        |index, tile| {
            if tile.construction == Construction::None {
                Some(link_add_street_to_town(town.id(), index))
            } else {
                None
            }
        },
    );

    render_buildings(data, &mut builder, renderer, town);
    render_streets_complex(renderer, town, |aabb, id, index| {
        builder.link(&link_remove_street_from_town(town.id(), index));

        if id.eq(&selected) {
            render_street_color(&mut builder, &aabb, Color::Yellow);
        } else {
            render_street(&mut builder, &aabb);
        }

        builder.close();
    });

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

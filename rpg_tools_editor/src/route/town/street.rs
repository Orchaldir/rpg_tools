use crate::html::create_html;
use crate::route::town::link_town_details;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::street::StreetId;
use rpg_tools_core::model::world::town::construction::Construction;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::create::street::add_street_to_tile;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::town::render_constructions;
use rpg_tools_rendering::usecase::map::TileMapRenderer;

#[get("/town/<id>/street/editor")]
pub fn get_street_editor(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_street_creator_html(&data, TownId::new(id))
}

pub fn link_street_creator(id: TownId) -> String {
    uri!(get_street_editor(id.id())).to_string()
}

#[get("/town/<id>/street/editor.svg")]
pub fn get_street_editor_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_town(&state.town_renderer, town))
}

#[get("/town/<id>/street/add/<tile>")]
pub fn add_street_to_town(
    state: &State<EditorData>,
    id: usize,
    tile: usize,
) -> Option<RawHtml<String>> {
    let mut data = state.data.lock().expect("lock shared data");
    let town_id = TownId::new(id);

    if add_street_to_tile(&mut data, town_id, tile, StreetId::default()).is_ok() {
        println!("Added a street to tile {} of town {}", tile, id);
    } else {
        println!("Failed to add a street to tile {} of town {}", tile, id);
    }

    get_street_creator_html(&data, town_id)
}

pub fn link_add_street_to_town(id: TownId, tile: usize) -> String {
    uri!(add_street_to_town(id.id(), tile)).to_string()
}

fn get_street_creator_html(data: &WorldData, id: TownId) -> Option<RawHtml<String>> {
    let map_uri = uri!(get_street_editor_map(id.id())).to_string();
    let back_uri = link_town_details(id);

    data.town_manager.get(id).map(|town| {
        let builder = create_html()
            .h1(&format!("Add Streets to Town {}", town.name()))
            .center(|b| b.svg(&map_uri, "800"))
            .p(|b| b.link(&back_uri, "Back"));

        RawHtml(builder.finish())
    })
}

fn render_town(renderer: &TileMapRenderer, town: &Town) -> RawSvg {
    let size = renderer.calculate_size(&town.map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles_with_link(
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

    render_constructions(&mut builder, renderer, town);

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

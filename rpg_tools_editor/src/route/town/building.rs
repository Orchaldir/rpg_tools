use crate::html::create_html;
use crate::route::town::link_town_details;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::building::lot::BuildingLot;
use rpg_tools_core::model::world::town::construction::Construction;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::create::building::create_building;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::town::{
    render_buildings, render_street, render_streets, render_streets_complex,
};
use rpg_tools_rendering::usecase::map::TileMapRenderer;

#[get("/town/<id>/building/creator")]
pub fn get_building_creator(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_building_creator_html(&data, TownId::new(id))
}

pub fn link_building_creator(id: TownId) -> String {
    uri!(get_building_creator(id.id())).to_string()
}

#[get("/town/<id>/building/creator.svg")]
pub fn get_building_creator_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_building_creator_map(&data, &state.town_renderer, town))
}

#[get("/town/<id>/building/add/<tile>")]
pub fn add_building(state: &State<EditorData>, id: usize, tile: usize) -> Option<RawHtml<String>> {
    let mut data = state.data.lock().expect("lock shared data");
    let town_id = TownId::new(id);

    if let Ok(building_id) = create_building(&mut data, BuildingLot::new(town_id, tile)) {
        println!(
            "Added building {} to tile {} of town {}",
            building_id.id(),
            tile,
            id
        );
    } else {
        println!("Failed to add a building to tile {} of town {}", tile, id);
    }

    get_building_creator_html(&data, town_id)
}

pub fn link_add_building(id: TownId, tile: usize) -> String {
    uri!(add_building(id.id(), tile)).to_string()
}

fn get_building_creator_html(data: &WorldData, id: TownId) -> Option<RawHtml<String>> {
    let map_uri = uri!(get_building_creator_map(id.id())).to_string();
    let back_uri = link_town_details(id);

    data.town_manager.get(id).map(|town| {
        let builder = create_html()
            .h1(&format!("Add a Building to Town {}", town.name()))
            .center(|b| b.svg(&map_uri, "800"))
            .p(|b| b.link(&back_uri, "Back"));

        RawHtml(builder.finish())
    })
}

fn render_building_creator_map(
    data: &WorldData,
    renderer: &TileMapRenderer,
    town: &Town,
) -> RawSvg {
    let size = renderer.calculate_map_size(&town.map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles_with_link(
        &mut builder,
        &Point2d::default(),
        &town.map,
        TownTile::get_color,
        |index, tile| {
            if tile.construction == Construction::None {
                Some(link_add_building(town.id(), index))
            } else {
                None
            }
        },
    );

    render_buildings(data, &mut builder, renderer, town);
    render_streets(&mut builder, renderer, town);

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

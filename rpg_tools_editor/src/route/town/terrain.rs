use crate::html::create_html;
use crate::route::town::link_town_details;
use crate::route::util::get_all_elements;
use crate::svg::RawSvg;
use crate::{EditorData, ToolData};
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::mountain::MountainId;
use rpg_tools_core::model::world::river::RiverId;
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::town::terrain::edit_terrain;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::town::render_constructs;
use rpg_tools_rendering::usecase::map::TileMapRenderer;

#[get("/town/<id>/terrain/editor")]
pub fn get_terrain_editor(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    let tools = state.tools.lock().expect("lock shared data");

    get_terrain_creator_html(&data, &tools, TownId::new(id))
}

pub fn link_terrain_editor(id: TownId) -> String {
    uri!(get_terrain_editor(id.id())).to_string()
}

#[derive(FromForm, Debug)]
pub struct TerrainEditorUpdate {
    terrain: String,
    id: Option<usize>,
}

#[post("/town/<id>/terrain/update", data = "<update>")]
pub fn update_terrain_editor(
    state: &State<EditorData>,
    id: usize,
    update: Form<TerrainEditorUpdate>,
) -> Option<RawHtml<String>> {
    println!("Update terrain editor {} with {:?}", id, update);
    let data = state.data.lock().expect("lock shared data");
    let mut tools = state.tools.lock().expect("lock shared data");

    tools.terrain = update.terrain.clone();
    tools.id = update.id;

    get_terrain_creator_html(&data, &tools, TownId::new(id))
}

#[get("/town/<id>/terrain/editor.svg")]
pub fn get_terrain_editor_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");

    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_terrain_editor_map(&data, &state.town_renderer, town))
}

#[get("/town/<id>/terrain/edit/<tile>")]
pub fn edit_terrain_route(
    state: &State<EditorData>,
    id: usize,
    tile: usize,
) -> Option<RawHtml<String>> {
    let mut data = state.data.lock().expect("lock shared data");
    let tools = state.tools.lock().expect("lock shared data");
    let town_id = TownId::new(id);
    let terrain = parse_terrain(&tools);

    if let Err(e) = edit_terrain(&mut data, town_id, tile, terrain.clone()) {
        println!(
            "Failed to change the terrain of tile {} of town {}: {}",
            tile, id, e
        );
    } else {
        println!(
            "Changed the terrain of tile {} of town {} to {:?}",
            tile, id, terrain,
        );
    }

    get_terrain_creator_html(&data, &tools, town_id)
}

pub fn link_edit_terrain(id: TownId, tile: usize) -> String {
    uri!(edit_terrain_route(id.id(), tile)).to_string()
}

fn get_terrain_creator_html(
    data: &WorldData,
    tools: &ToolData,
    id: TownId,
) -> Option<RawHtml<String>> {
    let map_uri = uri!(get_terrain_editor_map(id.id())).to_string();
    let back_uri = link_town_details(id);
    let update_uri = uri!(update_terrain_editor(id.id())).to_string();
    let mountains = get_all_elements(&data.mountain_manager);
    let rivers = get_all_elements(&data.river_manager);
    let terrain_id = tools.id.unwrap_or(0);

    data.town_manager.get(id).map(|town| {
        let builder = create_html()
            .h1(&format!("Add Streets to Town {}", town.name()))
            .form(&update_uri, |mut b| {
                b = b.select(
                    "Terrain",
                    "terrain",
                    &["Hill", "Mountain", "Plain", "River"],
                    &tools.terrain,
                );

                match tools.terrain.as_str() {
                    "Hill" => b.select_id("Hill", "id", &mountains, terrain_id),
                    "Mountain" => b.select_id("Mountain", "id", &mountains, terrain_id),
                    "River" => b.select_id("River", "id", &rivers, terrain_id),
                    _ => b,
                }
            })
            .center(|b| b.svg(&map_uri, "800"))
            .p(|b| b.link(&back_uri, "Back"));

        RawHtml(builder.finish())
    })
}

fn render_terrain_editor_map(data: &WorldData, renderer: &TileMapRenderer, town: &Town) -> RawSvg {
    let size = renderer.calculate_map_size(&town.map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles_with_link(
        &mut builder,
        &Point2d::default(),
        &town.map,
        TownTile::get_color,
        |index, _tile| Some(link_edit_terrain(town.id(), index)),
    );

    render_constructs(data, &mut builder, renderer, town);

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

fn parse_terrain(tools: &ToolData) -> Terrain {
    let terrain_id = tools.id.unwrap_or(0);

    match tools.terrain.as_str() {
        "Hill" => Terrain::Hill {
            id: MountainId::new(terrain_id),
        },
        "Mountain" => Terrain::Mountain {
            id: MountainId::new(terrain_id),
        },
        "River" => Terrain::River {
            id: RiverId::new(terrain_id),
        },
        _ => Terrain::Plain,
    }
}

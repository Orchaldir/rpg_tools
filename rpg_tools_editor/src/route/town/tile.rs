use crate::html::HtmlBuilder;
use crate::route::get_all_elements;
use crate::svg::RawSvg;
use crate::EditorData;
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
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::EdgeMapRenderer;

#[get("/town/<id>/tile/all")]
pub fn get_all_tiles(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data, TownId::new(id))
}

#[get("/town/<id>/tile/map.svg")]
pub fn get_tile_edit_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_to_svg(&state.town_renderer, town))
}

#[get("/town/<id>/tile/<index>/edit")]
pub fn edit_tile(state: &State<EditorData>, id: usize, index: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_template(&data, TownId::new(id), index)
}

#[derive(FromForm, Debug)]
pub struct TileUpdate<'r> {
    terrain: &'r str,
    id: Option<usize>,
}

#[post("/town/<id>/tile/<index>/preview", data = "<update>")]
pub fn preview_tile(
    state: &State<EditorData>,
    id: usize,
    index: usize,
    update: Form<TileUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Preview tile {} of town {} with {:?}", index, id, update);
    let data = state.data.lock().expect("lock shared data");

    let town_id = TownId::new(id);
    let tile = parse_tile(update);

    data.town_manager
        .get(town_id)
        .map(|town| get_form_template(&data, town_id, index, town, &tile))
}

fn parse_tile(update: Form<TileUpdate>) -> TownTile {
    let terrain_id = update.id.unwrap_or(0);
    TownTile::new(match update.terrain {
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
    })
}

#[post("/town/<id>/tile/<index>/update", data = "<update>")]
pub fn update_tile(
    state: &State<EditorData>,
    id: usize,
    index: usize,
    update: Form<TileUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update tile {} of town {} with {:?}", index, id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let town_id = TownId::new(id);

    if let Some(town) = data.town_manager.get_mut(town_id) {
        if let Some(old_tile) = town.map.get_tile_mut(index) {
            *old_tile = parse_tile(update);
        }
    }

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
        |index, _tile| uri!(edit_tile(town.id().id(), index)).to_string(),
    );

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

fn get_all_template(data: &WorldData, id: TownId) -> Option<RawHtml<String>> {
    let map_uri = uri!(get_tile_edit_map(id = id.id())).to_string();

    data.town_manager.get(id).map(|town| {
        let builder = HtmlBuilder::editor()
            .h1(&format!("Edit Terrain of Town {}", town.name()))
            .center(|b| b.svg(&map_uri, "800"))
            .p(|b| b.link(&format!("/town/{}/details", id.id()), "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_template(data: &WorldData, id: TownId, index: usize) -> Option<RawHtml<String>> {
    data.town_manager.get(id).and_then(|town| {
        town.map
            .get_tile(index)
            .map(|tile| get_form_template(data, id, index, town, tile))
    })
}

fn get_form_template(
    data: &WorldData,
    id: TownId,
    index: usize,
    town: &Town,
    tile: &TownTile,
) -> RawHtml<String> {
    let back_uri = uri!(get_all_tiles(id = id.id())).to_string();
    let mountains = get_all_elements(&data.mountain_manager);
    let rivers = get_all_elements(&data.river_manager);

    let builder = HtmlBuilder::editor()
        .h1(&format!("Edit Town Tile {} of {}", index, town.name()))
        .form(&format!("/town/{}/tile/{}", id.id(), index), |mut b| {
            let terrain = match tile.terrain {
                Terrain::Hill { .. } => "Hill",
                Terrain::Mountain { .. } => "Mountain",
                Terrain::Plain => "Plain",
                Terrain::River { .. } => "River",
            };
            b = b.select(
                "Terrain",
                "terrain",
                &["Hill", "Mountain", "Plain", "River"],
                terrain,
            );

            match tile.terrain {
                Terrain::Hill { id } => b.select_id("Hill", "id", &mountains, id.id()),
                Terrain::Mountain { id } => b.select_id("Mountain", "id", &mountains, id.id()),
                Terrain::Plain => b,
                Terrain::River { id } => b.select_id("River", "id", &rivers, id.id()),
            }
        })
        .p(|b| b.link(&back_uri, "Back"));

    RawHtml(builder.finish())
}

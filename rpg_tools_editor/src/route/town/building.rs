use crate::html::HtmlBuilder;
use crate::route::building::get_building_details_html;
use crate::route::town::link_town_details;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::building::lot::BuildingLot;
use rpg_tools_core::model::world::town::construction::Construction;
use rpg_tools_core::model::world::town::construction::Construction::Building;
use rpg_tools_core::model::world::town::tile::TownTile;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::create::building::create_building;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::style::RenderStyle;
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
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
        .map(|town| render_town(&state.town_renderer, town))
}

#[get("/town/<id>/building/add/<tile>")]
pub fn add_building(state: &State<EditorData>, id: usize, tile: usize) -> Option<RawHtml<String>> {
    let mut data = state.data.lock().expect("lock shared data");

    if let Ok(building_id) = create_building(
        &mut data,
        BuildingLot {
            town: TownId::new(id),
            tile,
        },
    ) {
        return get_building_details_html(&data, building_id);
    }

    get_building_creator(state, id)
}

pub fn link_add_building(id: TownId, tile: usize) -> String {
    uri!(add_building(id.id(), tile)).to_string()
}

fn get_building_creator_html(data: &WorldData, id: TownId) -> Option<RawHtml<String>> {
    let map_uri = uri!(get_building_creator_map(id.id())).to_string();
    let back_uri = link_town_details(id);

    data.town_manager.get(id).map(|town| {
        let builder = HtmlBuilder::editor()
            .h1(&format!("Add a Building to Town {}", town.name()))
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
                Some(link_add_building(town.id(), index))
            } else {
                None
            }
        },
    );

    renderer.render(&Point2d::default(), &town.map, |_index, aabb, tile| {
        if let Building { .. } = tile.construction {
            let style = RenderStyle::no_border(Color::Black);
            builder.render_rectangle(&aabb.scale(0.5), &style);
        }
    });

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

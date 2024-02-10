use crate::route::get_all_template;
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::world::town::edge::TownEdge;
use rpg_tools_core::model::world::town::terrain::Terrain;
use rpg_tools_core::model::world::town::{Town, TownId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::storage::{Element, Id};
use rpg_tools_rendering::renderer::svg::builder::SvgBuilder;
use rpg_tools_rendering::usecase::map::EdgeMapRenderer;

#[get("/town/all")]
pub fn get_all_towns(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    get_all_template(&data.town_manager, "town", "Towns")
}

#[get("/town/details/<id>")]
pub fn get_town_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    get_details_template(&data, TownId::new(id))
}

#[get("/town/map/<id>/map.svg")]
pub fn get_town_map(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.town_manager
        .get(TownId::new(id))
        .map(|town| render_to_svg(&state.town_renderer, town))
}

fn get_details_template(data: &WorldData, id: TownId) -> Option<Template> {
    data.town_manager.get(id).map(|town| {
        Template::render(
            "town/details",
            context! {
                name: town.name(),
                id: id.id(),
            },
        )
    })
}

pub fn render_to_svg(renderer: &EdgeMapRenderer, town: &Town) -> RawSvg {
    let size = renderer.calculate_size(&town.map);
    let mut builder = SvgBuilder::new(size);

    renderer.render_tiles(
        &mut builder,
        &Point2d::default(),
        &town.map,
        |tile| match tile.terrain {
            Terrain::Hill { .. } => Color::SaddleBrown,
            Terrain::Mountain { .. } => Color::Gray,
            Terrain::Plain => Color::Green,
            Terrain::River { .. } => Color::Blue,
        },
    );

    renderer.render_edges(
        &mut builder,
        &Point2d::default(),
        &town.map,
        |tile| match tile {
            TownEdge::None => None,
            TownEdge::Street { .. } => Some(Color::White),
        },
    );

    let svg = builder.finish();
    RawSvg::new(svg.export())
}

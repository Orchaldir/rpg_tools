use crate::html::HtmlBuilder;
use crate::route::building::link_building_details;
use crate::route::town::{link_town_details, render_town};
use crate::svg::RawSvg;
use crate::EditorData;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::town::TownId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::storage::{Element, Id};

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
        .map(|town| render_town(&state.town_renderer, town, link_building_details))
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

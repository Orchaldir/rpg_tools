use crate::html::HtmlBuilder;
use crate::route::get_all_template;
use crate::route::town::link_town_details;
use crate::EditorData;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::model::world::building::BuildingId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/building/all")]
pub fn get_all_buildings(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data.building_manager, "building", "Buildings")
}

pub fn link_all_buildings() -> String {
    uri!(get_all_buildings()).to_string()
}

#[get("/building/<id>/details")]
pub fn get_building_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_template(&data, BuildingId::new(id))
}

fn get_details_template(data: &WorldData, id: BuildingId) -> Option<RawHtml<String>> {
    data.building_manager.get(id).map(|building| {
        let mut builder = HtmlBuilder::editor()
            .h1(&format!("Building: {}", building.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field("Name:", building.name())
            .h3("Lot");

        if let Some(town) = data.town_manager.get(building.lot().town) {
            builder = builder.complex_field("Town", |b| {
                b.link(&link_town_details(town.id()), town.name())
            })
        }

        builder = builder
            .field_usize("Tile:", building.lot().tile)
            .field_size2d("Size:", &Size2d::square(1))
            .p(|b| b.link(&link_all_buildings(), "Back"));

        RawHtml(builder.finish())
    })
}

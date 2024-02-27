use crate::html::{create_html, EditorBuilder};
use crate::route::town::link_town_details;
use crate::route::util::get_all_html;
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::building::BuildingId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::usecase::edit::resize::resize_building;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/building/all")]
pub fn get_all_buildings(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_html(&data.building_manager, "Buildings")
}

pub fn link_all_buildings() -> String {
    uri!(get_all_buildings()).to_string()
}

#[get("/building/<id>/details")]
pub fn get_building_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_building_details_html(&data, BuildingId::new(id))
}

pub fn link_building_details(id: BuildingId) -> String {
    uri!(get_building_details(id.id())).to_string()
}

#[get("/building/<id>/edit")]
pub fn edit_building(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_html(&data, BuildingId::new(id), "", "")
}

pub fn link_edit_building(id: BuildingId) -> String {
    uri!(edit_building(id.id())).to_string()
}

#[derive(FromForm, Debug)]
pub struct BuildingUpdate<'r> {
    name: &'r str,
    width: u32,
    height: u32,
}

#[post("/building/<id>/update", data = "<update>")]
pub fn update_building(
    state: &State<EditorData>,
    id: usize,
    update: Form<BuildingUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update mountain {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let building_id = BuildingId::new(id);

    if let Err(e) = update_name(&mut data.building_manager, building_id, update.name) {
        return get_edit_html(&data, building_id, &e.to_string(), "");
    } else if let Err(e) = resize_building(&mut data, building_id, update.width, update.height) {
        return get_edit_html(&data, building_id, "", &e.to_string());
    }

    get_building_details_html(&data, building_id)
}

pub fn get_building_details_html(data: &WorldData, id: BuildingId) -> Option<RawHtml<String>> {
    data.building_manager.get(id).map(|building| {
        let mut builder = create_html()
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
            .field_size2d("Size:", &building.lot().size)
            .p(|b| b.link(&link_edit_building(id), "Edit"))
            .p(|b| b.link(&link_all_buildings(), "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_html(
    data: &WorldData,
    id: BuildingId,
    name_error: &str,
    size_error: &str,
) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_building(id.id())).to_string();

    data.building_manager.get(id).map(|building| {
        let builder = create_html()
            .h1(&format!("Edit Building: {}", building.name()))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input("Name", "name", building.name())
                    .error(name_error)
                    .number_input("Width", "width", building.lot.size.width() as usize, 1, 100)
                    .number_input(
                        "Height",
                        "height",
                        building.lot.size.height() as usize,
                        1,
                        100,
                    )
                    .error(size_error)
            })
            .p(|b| b.link(&link_building_details(id), "Back"));

        RawHtml(builder.finish())
    })
}

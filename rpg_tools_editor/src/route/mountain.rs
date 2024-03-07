use crate::html::create_html;
use crate::route::town::link_town_details;
use crate::route::util::{get_all_html, get_elements};
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::mountain::{Mountain, MountainId};
use rpg_tools_core::model::world::town::towns::WithTowns;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/mountain/all")]
pub fn get_all_mountains(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_html(&data.mountain_manager, "Mountains")
}

pub fn link_all_mountains() -> String {
    uri!(get_all_mountains()).to_string()
}

#[get("/mountain/new")]
pub fn add_mountain(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.mountain_manager.create(Mountain::new);

    println!("Create mountain {}", id.id());

    get_edit_html(&data, id, "")
}

#[get("/mountain/<id>/details")]
pub fn get_mountain_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_html(&data, MountainId::new(id))
}

pub fn link_mountain_details(id: MountainId) -> String {
    uri!(get_mountain_details(id = id.id())).to_string()
}

#[get("/mountain/<id>/edit")]
pub fn edit_mountain(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_html(&data, MountainId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct MountainUpdate<'r> {
    name: &'r str,
}

#[post("/mountain/<id>/update", data = "<update>")]
pub fn update_mountain(
    state: &State<EditorData>,
    id: usize,
    update: Form<MountainUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update mountain {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let mountain_id = MountainId::new(id);

    if let Err(e) = update_name(&mut data.mountain_manager, mountain_id, update.name) {
        return get_edit_html(&data, mountain_id, &e.to_string());
    }

    get_details_html(&data, mountain_id)
}

fn get_details_html(data: &WorldData, id: MountainId) -> Option<RawHtml<String>> {
    let edit_uri = uri!(edit_mountain(id = id.id())).to_string();

    data.mountain_manager.get(id).map(|mountain| {
        let towns = get_elements(&data.town_manager, mountain.towns());

        let builder = create_html()
            .h1(&format!("Mountain: {}", mountain.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field_usize("Towns:", towns.len())
            .list(&towns, |b, &town| {
                b.link(&link_town_details(town.id()), town.name())
            })
            .p(|b| b.link(&edit_uri, "Edit"))
            .p(|b| b.link(&link_all_mountains(), "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_html(data: &WorldData, id: MountainId, name_error: &str) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_mountain(id.id())).to_string();

    data.mountain_manager.get(id).map(|mountain| {
        let builder = create_html()
            .h1(&format!("Edit Mountain: {}", mountain.name()))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input("Name", "name", mountain.name())
                    .error(name_error)
            })
            .p(|b| b.link(&link_mountain_details(id), "Back"));

        RawHtml(builder.finish())
    })
}

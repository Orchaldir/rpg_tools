use crate::html::create_html;
use crate::route::town::link_town_details;
use crate::route::util::{get_all_html, get_elements};
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::name::WithName;
use rpg_tools_core::model::world::river::{River, RiverId};
use rpg_tools_core::model::world::town::towns::WithTowns;
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/river/all")]
pub fn get_all_rivers(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_html(&data.river_manager, "Rivers")
}

pub fn link_all_rivers() -> String {
    uri!(get_all_rivers()).to_string()
}

#[get("/river/new")]
pub fn add_river(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.river_manager.create(River::new);

    println!("Create river {}", id.id());

    get_edit_html(&data, id, "")
}

#[get("/river/<id>/details")]
pub fn get_river_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_html(&data, RiverId::new(id))
}

pub fn link_river_details(id: RiverId) -> String {
    uri!(get_river_details(id = id.id())).to_string()
}

#[get("/river/<id>/edit")]
pub fn edit_river(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_html(&data, RiverId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct RiverUpdate<'r> {
    name: &'r str,
}

#[post("/river/<id>/update", data = "<update>")]
pub fn update_river(
    state: &State<EditorData>,
    id: usize,
    update: Form<RiverUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update river {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let river_id = RiverId::new(id);

    if let Err(e) = update_name(&mut data.river_manager, river_id, update.name) {
        return get_edit_html(&data, river_id, &e.to_string());
    }

    get_details_html(&data, river_id)
}

fn get_details_html(data: &RpgData, id: RiverId) -> Option<RawHtml<String>> {
    let edit_uri = uri!(edit_river(id = id.id())).to_string();

    data.river_manager.get(id).map(|river| {
        let towns = get_elements(&data.town_manager, river.towns());

        let builder = create_html()
            .h1(&format!("River: {}", river.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field_usize("Towns:", towns.len())
            .list(&towns, |b, &town| {
                b.link(&link_town_details(town.id()), town.name().str())
            })
            .p(|b| b.link(&edit_uri, "Edit"))
            .p(|b| b.link(&link_all_rivers(), "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_html(data: &RpgData, id: RiverId, name_error: &str) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_river(id.id())).to_string();

    data.river_manager.get(id).map(|river| {
        let builder = create_html()
            .h1(&format!("Edit River: {}", river.name()))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input("Name", "name", river.name().str())
                    .error(name_error)
            })
            .p(|b| b.link(&link_river_details(id), "Back"));

        RawHtml(builder.finish())
    })
}

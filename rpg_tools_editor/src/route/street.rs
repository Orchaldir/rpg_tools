use crate::html::create_html;
use crate::route::town::link_town_details;
use crate::route::util::{get_all_html, get_elements};
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::street::{Street, StreetId};
use rpg_tools_core::model::world::town::towns::WithTowns;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/street/all")]
pub fn get_all_streets(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_html(&data.street_manager, "Streets")
}

#[get("/street/new")]
pub fn add_street(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.street_manager.create(Street::new);

    println!("Create street {}", id.id());

    get_edit_html(&data, id, "")
}

#[get("/street/<id>/details")]
pub fn get_street_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_html(&data, StreetId::new(id))
}

#[get("/street/<id>/edit")]
pub fn edit_street(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_html(&data, StreetId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct StreetUpdate<'r> {
    name: &'r str,
}

#[post("/street/<id>/update", data = "<update>")]
pub fn update_street(
    state: &State<EditorData>,
    id: usize,
    update: Form<StreetUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update street {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let street_id = StreetId::new(id);

    if let Err(e) = update_name(&mut data.street_manager, street_id, update.name) {
        return get_edit_html(&data, street_id, &e.to_string());
    }

    get_details_html(&data, street_id)
}

fn get_details_html(data: &WorldData, id: StreetId) -> Option<RawHtml<String>> {
    data.street_manager.get(id).map(|street| {
        let towns = get_elements(&data.town_manager, street.towns());

        let builder = create_html()
            .h1(&format!("Street: {}", street.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field_usize("Towns:", towns.len())
            .list(&towns, |b, &town| {
                b.link(&link_town_details(town.id()), town.name())
            })
            .p(|b| b.link(&format!("/street/{}/edit", id.id()), "Edit"))
            .p(|b| b.link("/street/all", "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_html(data: &WorldData, id: StreetId, name_error: &str) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_street(id.id())).to_string();

    data.street_manager.get(id).map(|street| {
        let builder = create_html()
            .h1(&format!("Edit Street: {}", street.name()))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input("Name", "name", street.name())
                    .error(name_error)
            })
            .p(|b| b.link(&format!("/street/{}/details", id.id()), "Back"));

        RawHtml(builder.finish())
    })
}

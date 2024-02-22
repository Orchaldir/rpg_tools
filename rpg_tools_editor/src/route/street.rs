use crate::html::HtmlBuilder;
use crate::route::{get_all_template, get_elements};
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::street::StreetId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/street/all")]
pub fn get_all_streets(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data.street_manager, "street", "Streets")
}

#[get("/street/details/<id>")]
pub fn get_street_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_template(&data, StreetId::new(id))
}

#[get("/street/new")]
pub fn add_street(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.street_manager.create();

    println!("Create street {}", id.id());

    get_edit_template(&data, id, "")
}

#[get("/street/edit/<id>")]
pub fn edit_street(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_template(&data, StreetId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct StreetUpdate<'r> {
    name: &'r str,
}

#[post("/street/update/<id>", data = "<update>")]
pub fn update_street(
    state: &State<EditorData>,
    id: usize,
    update: Form<StreetUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update street {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let street_id = StreetId::new(id);

    if let Err(e) = update_name(&mut data.street_manager, street_id, update.name) {
        return get_edit_template(&data, street_id, &e.to_string());
    }

    get_details_template(&data, street_id)
}

fn get_details_template(data: &WorldData, id: StreetId) -> Option<RawHtml<String>> {
    data.street_manager.get(id).map(|street| {
        let towns = get_elements(&data.town_manager, &street.towns);

        let builder = HtmlBuilder::editor()
            .h1(&format!("Street: {}", street.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field_usize("Towns:", towns.len())
            .list(data.town_manager.get_all(), |b, e| {
                b.link(&format!("/town/details/{}", e.id().id()), e.name())
            })
            .p(|b| b.link(&format!("/street/edit/{}", id.id()), "Edit"))
            .p(|b| b.link("/street/all", "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_template(data: &WorldData, id: StreetId, name_error: &str) -> Option<RawHtml<String>> {
    data.street_manager.get(id).map(|street| {
        let builder = HtmlBuilder::editor()
            .h1(&format!("Edit Street: {}", street.name()))
            .field_usize("Id:", id.id())
            .form(&format!("/street/update/{}", id.id()), |b| {
                b.text_input("Name", "name", street.name())
                    .error(name_error)
            })
            .p(|b| b.link(&format!("/street/details/{}", id.id()), "Back"));

        RawHtml(builder.finish())
    })
}

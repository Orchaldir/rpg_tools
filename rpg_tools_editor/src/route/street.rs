use crate::route::{get_all_template, get_elements};
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::street::StreetId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/street/all")]
pub fn get_all_streets(state: &State<EditorData>) -> Template {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data.street_manager, "street", "Streets")
}

#[get("/street/details/<id>")]
pub fn get_street_details(state: &State<EditorData>, id: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    get_details_template(&data, StreetId::new(id))
}

#[get("/street/new")]
pub fn add_street(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.street_manager.create();

    println!("Create street {}", id.id());

    get_edit_template(&data, id, "")
}

#[get("/street/edit/<id>")]
pub fn edit_street(state: &State<EditorData>, id: usize) -> Option<Template> {
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
) -> Option<Template> {
    println!("Update street {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let street_id = StreetId::new(id);

    if let Err(e) = update_name(&mut data.street_manager, street_id, update.name) {
        return get_edit_template(&data, street_id, &e.to_string());
    }

    get_details_template(&data, street_id)
}

fn get_details_template(data: &WorldData, id: StreetId) -> Option<Template> {
    data.street_manager.get(id).map(|street| {
        let towns = get_elements(&data.town_manager, &street.towns);

        Template::render(
            "street/details",
            context! {
                name: street.name(),
                id: id.id(),
                towns: towns,
            },
        )
    })
}

fn get_edit_template(data: &WorldData, id: StreetId, name_error: &str) -> Option<Template> {
    data.street_manager.get(id).map(|street| {
        Template::render(
            "street/edit",
            context! {
                name: street.name(),
                id: id.id(),
                name_error: name_error,
            },
        )
    })
}

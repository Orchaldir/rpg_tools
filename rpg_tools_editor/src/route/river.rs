use crate::html::HtmlBuilder;
use crate::route::{get_all_template, get_elements};
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::river::RiverId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/river/all")]
pub fn get_all_rivers(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data.river_manager, "river", "Rivers")
}

#[get("/river/details/<id>")]
pub fn get_river_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_template(&data, RiverId::new(id))
}

#[get("/river/new")]
pub fn add_river(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.river_manager.create();

    println!("Create river {}", id.id());

    get_edit_template(&data, id, "")
}

#[get("/river/edit/<id>")]
pub fn edit_river(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_template(&data, RiverId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct RiverUpdate<'r> {
    name: &'r str,
}

#[post("/river/update/<id>", data = "<update>")]
pub fn update_river(
    state: &State<EditorData>,
    id: usize,
    update: Form<RiverUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update river {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let river_id = RiverId::new(id);

    if let Err(e) = update_name(&mut data.river_manager, river_id, update.name) {
        return get_edit_template(&data, river_id, &e.to_string());
    }

    get_details_template(&data, river_id)
}

fn get_details_template(data: &WorldData, id: RiverId) -> Option<RawHtml<String>> {
    data.river_manager.get(id).map(|river| {
        let towns = get_elements(&data.town_manager, &river.towns);

        let builder = HtmlBuilder::editor()
            .h1(&format!("River: {}", river.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field_usize("Towns:", towns.len())
            .list(data.town_manager.get_all(), |b, e| {
                b.link(&format!("/town/details/{}", e.id().id()), e.name())
            })
            .p(|b| b.link(&format!("/river/edit/{}", id.id()), "Edit"))
            .p(|b| b.link("/river/all", "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_template(data: &WorldData, id: RiverId, name_error: &str) -> Option<RawHtml<String>> {
    data.river_manager.get(id).map(|river| {
        let builder = HtmlBuilder::editor()
            .h1(&format!("Edit River: {}", river.name()))
            .field_usize("Id:", id.id())
            .form(&format!("/river/update/{}", id.id()), |b| {
                b.text_input("Name", "name", river.name()).error(name_error)
            })
            .p(|b| b.link(&format!("/river/details/{}", id.id()), "Back"));

        RawHtml(builder.finish())
    })
}

use crate::route::{get_all_template, get_elements};
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::river::RiverId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/river/all")]
pub fn get_all_rivers(state: &State<EditorData>) -> Template {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data.river_manager, "river", "Rivers")
}

#[get("/river/details/<id>")]
pub fn get_river_details(state: &State<EditorData>, id: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    get_details_template(&data, RiverId::new(id))
}

#[get("/river/edit/<id>")]
pub fn edit_river(state: &State<EditorData>, id: usize) -> Option<Template> {
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
) -> Option<Template> {
    println!("Update river {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let river_id = RiverId::new(id);

    if let Err(e) = update_name(&mut data.river_manager, river_id, update.name) {
        return get_edit_template(&data, river_id, &e.to_string());
    }

    get_details_template(&data, river_id)
}

fn get_details_template(data: &WorldData, id: RiverId) -> Option<Template> {
    data.river_manager.get(id).map(|river| {
        let towns = get_elements(&data.town_manager, &river.towns);

        Template::render(
            "river/details",
            context! {
                name: river.name(),
                id: id.id(),
                towns: towns,
            },
        )
    })
}

fn get_edit_template(data: &WorldData, id: RiverId, name_error: &str) -> Option<Template> {
    data.river_manager.get(id).map(|river| {
        Template::render(
            "river/edit",
            context! {
                name: river.name(),
                id: id.id(),
                name_error: name_error,
            },
        )
    })
}

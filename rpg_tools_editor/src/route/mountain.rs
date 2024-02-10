use crate::route::get_all_template;
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::mountain::MountainId;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/mountain/all")]
pub fn get_all_mountains(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.mountain_manager, "mountain", "Mountains")
}

#[get("/mountain/details/<id>")]
pub fn get_mountain_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    get_details_template(data, MountainId::new(id))
}

#[get("/mountain/edit/<id>")]
pub fn edit_mountain(data: &State<EditorData>, id: usize) -> Option<Template> {
    get_edit_template(data, MountainId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct MountainUpdate<'r> {
    name: &'r str,
}

#[post("/mountain/update/<id>", data = "<update>")]
pub fn update_mountain(
    data: &State<EditorData>,
    id: usize,
    update: Form<MountainUpdate<'_>>,
) -> Option<Template> {
    println!("Update mountain {} with {:?}", id, update);

    let mountain_id = MountainId::new(id);

    /*
    if let Err(e) = update_mountain_name(&mut data, mountain_id, update.name) {
        return get_edit_template(data, mountain_id, &e.to_string());
    }

    get_details_template(data, mountain_id)
    */
    get_edit_template(data, mountain_id, "test error!")
}

fn get_details_template(data: &EditorData, id: MountainId) -> Option<Template> {
    data.data.mountain_manager.get(id).map(|mountain| {
        Template::render(
            "mountain/details",
            context! {
                name: mountain.name(),
                id: id.id(),
            },
        )
    })
}

fn get_edit_template(data: &EditorData, id: MountainId, name_error: &str) -> Option<Template> {
    data.data.mountain_manager.get(id).map(|mountain| {
        Template::render(
            "mountain/edit",
            context! {
                name: mountain.name(),
                id: id.id(),
                name_error: name_error,
            },
        )
    })
}

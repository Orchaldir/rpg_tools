use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::mountain::{Mountain, MountainId};
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/mountain/all")]
pub fn get_all_mountains(data: &State<EditorData>) -> Template {
    crate::route::get_all_template(&data.data.mountain_manager, "mountain", "Mountains")
}

#[get("/mountain/details/<id>")]
pub fn get_mountain_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    data.data
        .mountain_manager
        .get(MountainId::new(id))
        .map(|mountain| get_details_template(&data, id, mountain))
}

fn get_details_template(data: &EditorData, id: usize, mountain: &Mountain) -> Template {
    Template::render(
        "mountain/details",
        context! {
            name: mountain.name(),
            id: id,
        },
    )
}

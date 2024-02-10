use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::river::{River, RiverId};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::utils::storage::{Element, Id, Storage};

#[get("/river/all")]
pub fn get_all_rivers(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.river_manager)
}

fn get_all_template<ID: Id, ELEMENT: Element<ID>>(storage: &Storage<ID, ELEMENT>) -> Template {
    let values: Vec<(usize, &str)> = storage
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name()))
        .collect();

    Template::render(
        "generic/all",
        context! {
            name: "river",
            title: "Rivers",
            values: values,
        },
    )
}

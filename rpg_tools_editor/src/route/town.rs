use crate::route::get_all_template;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::town::TownId;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/town/all")]
pub fn get_all_towns(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.town_manager, "town", "Towns")
}

#[get("/town/details/<id>")]
pub fn get_town_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    get_details_template(data, TownId::new(id))
}

fn get_details_template(data: &EditorData, id: TownId) -> Option<Template> {
    data.data.town_manager.get(id).map(|town| {
        Template::render(
            "town/details",
            context! {
                name: town.name(),
                id: id.id(),
            },
        )
    })
}

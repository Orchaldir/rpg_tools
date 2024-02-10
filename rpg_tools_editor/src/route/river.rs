use crate::route::get_all_template;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::river::RiverId;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/river/all")]
pub fn get_all_rivers(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.river_manager, "river", "Rivers")
}

#[get("/river/details/<id>")]
pub fn get_river_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    get_details_template(data, RiverId::new(id))
}

fn get_details_template(data: &EditorData, id: RiverId) -> Option<Template> {
    data.data.river_manager.get(id).map(|river| {
        let towns: Vec<(usize, &str)> = river
            .towns
            .iter()
            .flat_map(|id| data.data.town_manager.get(*id))
            .map(|c| (c.id().id(), c.name()))
            .collect();

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

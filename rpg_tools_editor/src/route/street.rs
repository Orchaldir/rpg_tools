use crate::route::{get_all_template, get_elements};
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::street::StreetId;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/street/all")]
pub fn get_all_streets(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.street_manager, "street", "Streets")
}

#[get("/street/details/<id>")]
pub fn get_street_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    get_details_template(data, StreetId::new(id))
}

fn get_details_template(data: &EditorData, id: StreetId) -> Option<Template> {
    data.data.street_manager.get(id).map(|street| {
        let towns = get_elements(&data.data.town_manager, &street.towns);

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

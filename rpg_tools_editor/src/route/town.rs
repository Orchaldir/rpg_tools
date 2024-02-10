use crate::route::get_all_template;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::Template;

#[get("/town/all")]
pub fn get_all_towns(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.town_manager, "town", "Towns")
}

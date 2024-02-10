use crate::route::get_all_template;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::Template;

#[get("/river/all")]
pub fn get_all_rivers(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.river_manager, "river", "Rivers")
}

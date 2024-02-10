use crate::route::get_all_template;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::Template;

#[get("/street/all")]
pub fn get_all_streets(data: &State<EditorData>) -> Template {
    get_all_template(&data.data.street_manager, "street", "Streets")
}

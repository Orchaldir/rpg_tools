use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::Template;

#[get("/mountain/all")]
pub fn get_all_mountains(data: &State<EditorData>) -> Template {
    crate::route::get_all_template(&data.data.mountain_manager, "mountain", "Mountains")
}

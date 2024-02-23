use crate::route::get_all_template;
use crate::EditorData;
use rocket::response::content::RawHtml;
use rocket::State;

#[get("/building/all")]
pub fn get_all_buildings(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data.building_manager, "building", "Buildings")
}

use crate::html::HtmlBuilder;
use crate::route::get_all_template;
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::mountain::MountainId;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/mountain/all")]
pub fn get_all_mountains(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_template(&data.mountain_manager, "mountain", "Mountains")
}

#[get("/mountain/details/<id>")]
pub fn get_mountain_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_template(&data, MountainId::new(id))
}

#[get("/mountain/new")]
pub fn add_mountain(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.mountain_manager.create();

    println!("Create mountain {}", id.id());

    get_edit_template(&data, id, "")
}

#[get("/mountain/edit/<id>")]
pub fn edit_mountain(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_template(&data, MountainId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct MountainUpdate<'r> {
    name: &'r str,
}

#[post("/mountain/update/<id>", data = "<update>")]
pub fn update_mountain(
    state: &State<EditorData>,
    id: usize,
    update: Form<MountainUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update mountain {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let mountain_id = MountainId::new(id);

    if let Err(e) = update_name(&mut data.mountain_manager, mountain_id, update.name) {
        return get_edit_template(&data, mountain_id, &e.to_string());
    }

    get_details_template(&data, mountain_id)
}

fn get_details_template(data: &WorldData, id: MountainId) -> Option<RawHtml<String>> {
    data.mountain_manager.get(id).map(|mountain| {
        let builder = HtmlBuilder::editor()
            .h1(&format!("Mountain: {}", mountain.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .p(|b| b.link(&format!("/mountain/edit/{}", id.id()), "Edit"))
            .p(|b| b.link("/mountain/all", "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_template(
    data: &WorldData,
    id: MountainId,
    name_error: &str,
) -> Option<RawHtml<String>> {
    data.mountain_manager.get(id).map(|mountain| {
        let builder = HtmlBuilder::editor()
            .h1(&format!("Edit Mountain: {}", mountain.name()))
            .field_usize("Id:", id.id())
            .form(&format!("/mountain/update/{}", id.id()), |b| {
                b.text_input("Name", "name", mountain.name())
                    .error(name_error)
            })
            .p(|b| b.link(&format!("/mountain/details/{}", id.id()), "Back"));

        RawHtml(builder.finish())
    })
}

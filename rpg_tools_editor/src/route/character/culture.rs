use crate::html::create_html;
use crate::route::character::link_character_details;
use crate::route::util::get_all_html;
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::character::culture::{Culture, CultureId};
use rpg_tools_core::model::name::WithName;
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/culture/all")]
pub fn get_all_cultures(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_html(&data.cultures, "Cultures")
}

pub fn link_all_cultures() -> String {
    uri!(get_all_cultures()).to_string()
}

#[get("/culture/new")]
pub fn add_culture(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.cultures.create(Culture::new);

    println!("Create culture {}", id.id());

    get_edit_html(&data, id, "")
}

#[get("/culture/<id>/details")]
pub fn get_culture_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_html(&data, CultureId::new(id))
}

pub fn link_culture_details(id: CultureId) -> String {
    uri!(get_culture_details(id = id.id())).to_string()
}

#[get("/culture/<id>/edit")]
pub fn edit_culture(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_html(&data, CultureId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct CultureUpdate<'r> {
    name: &'r str,
}

#[post("/culture/<id>/update", data = "<update>")]
pub fn update_culture(
    state: &State<EditorData>,
    id: usize,
    update: Form<CultureUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update culture {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let culture_id = CultureId::new(id);

    if let Err(e) = update_name(&mut data.cultures, culture_id, update.name) {
        return get_edit_html(&data, culture_id, &e.to_string());
    }

    get_details_html(&data, culture_id)
}

fn get_details_html(data: &RpgData, id: CultureId) -> Option<RawHtml<String>> {
    let edit_uri = uri!(edit_culture(id = id.id())).to_string();

    data.cultures.get(id).map(|culture| {
        let characters: Vec<_> = data
            .characters
            .get_all()
            .iter()
            .filter(|c| c.culture.eq(&id))
            .map(|c| (c.id(), c.name.to_string()))
            .collect();

        let builder = create_html()
            .h1(&format!("Culture: {}", culture.name().str()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field_usize("Characters:", characters.len())
            .list(&characters, |b, character| {
                b.link(&link_character_details(character.0), &character.1)
            })
            .p(|b| b.link(&edit_uri, "Edit"))
            .p(|b| b.link(&link_all_cultures(), "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_html(data: &RpgData, id: CultureId, name_error: &str) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_culture(id.id())).to_string();

    data.cultures.get(id).map(|culture| {
        let builder = create_html()
            .h1(&format!("Edit Culture: {}", culture.name().str()))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input("Name", "name", culture.name().str())
                    .error(name_error)
            })
            .p(|b| b.link(&link_culture_details(id), "Back"));

        RawHtml(builder.finish())
    })
}

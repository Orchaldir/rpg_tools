use crate::html::create_html;
use crate::route::util::get_all_html;
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::edit::name::update_name;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/character/all")]
pub fn get_all_characters(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    get_all_html(&data.characters, "Characters")
}

pub fn link_all_characters() -> String {
    uri!(get_all_characters()).to_string()
}

#[get("/character/new")]
pub fn add_character(data: &State<EditorData>) -> Option<RawHtml<String>> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.characters.create(Character::new);

    println!("Create character {}", id.id());

    get_edit_html(&data, id, "")
}

#[get("/character/<id>/details")]
pub fn get_character_details(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_details_html(&data, CharacterId::new(id))
}

pub fn link_character_details(id: CharacterId) -> String {
    uri!(get_character_details(id = id.id())).to_string()
}

#[get("/character/<id>/edit")]
pub fn edit_character(state: &State<EditorData>, id: usize) -> Option<RawHtml<String>> {
    let data = state.data.lock().expect("lock shared data");
    get_edit_html(&data, CharacterId::new(id), "")
}

#[derive(FromForm, Debug)]
pub struct CharacterUpdate<'r> {
    name: &'r str,
    gender: &'r str,
}

#[post("/character/<id>/update", data = "<update>")]
pub fn update_character(
    state: &State<EditorData>,
    id: usize,
    update: Form<CharacterUpdate<'_>>,
) -> Option<RawHtml<String>> {
    println!("Update character {} with {:?}", id, update);
    let mut data = state.data.lock().expect("lock shared data");

    let character_id = CharacterId::new(id);

    if let Err(e) = update_name(&mut data.characters, character_id, update.name) {
        return get_edit_html(&data, character_id, &e.to_string());
    }

    get_details_html(&data, character_id)
}

fn get_details_html(data: &RpgData, id: CharacterId) -> Option<RawHtml<String>> {
    let edit_uri = uri!(edit_character(id = id.id())).to_string();

    data.characters.get(id).map(|character| {
        let builder = create_html()
            .h1(&format!("Character: {}", character.name()))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field("Gender:", &character.gender.to_string())
            .p(|b| b.link(&edit_uri, "Edit"))
            .p(|b| b.link(&link_all_characters(), "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_html(data: &RpgData, id: CharacterId, name_error: &str) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_character(id.id())).to_string();

    data.characters.get(id).map(|character| {
        let builder = create_html()
            .h1(&format!("Edit Character: {}", character.name()))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input("Name", "name", character.name())
                    .error(name_error)
                    .select(
                        "Gender:",
                        "gender",
                        &["Female", "Genderless", "Male"],
                        &character.gender.to_string(),
                    )
            })
            .p(|b| b.link(&link_character_details(id), "Back"));

        RawHtml(builder.finish())
    })
}

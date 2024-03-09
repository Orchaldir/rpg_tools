pub mod culture;

use crate::html::create_html;
use crate::route::character::culture::link_culture_details;
use crate::route::link_home;
use crate::EditorData;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::character::name::Lastname;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::name::WithName;
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::edit::character::gender::update_gender;
use rpg_tools_core::utils::storage::{Element, Id};

#[get("/character/all")]
pub fn get_all_characters(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    let new_uri = uri!(add_character()).to_string();

    RawHtml(
        create_html()
            .h1("Characters")
            .field("Count:", &data.characters.len().to_string())
            .list(data.characters.get_all(), |b, e| {
                b.link(&link_character_details(e.id()), &e.name.to_string())
            })
            .p(|b| b.link(&new_uri, "Add"))
            .p(|b| b.link(&link_home(), "Back"))
            .finish(),
    )
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
    first_name: &'r str,
    middle_name: &'r str,
    last_type: &'r str,
    last_name: &'r str,
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

    if let Err(e) = update_gender(&mut data, character_id, update.gender.into()) {
        return get_edit_html(&data, character_id, &e.to_string());
    }

    get_details_html(&data, character_id)
}

fn get_details_html(data: &RpgData, id: CharacterId) -> Option<RawHtml<String>> {
    let edit_uri = uri!(edit_character(id = id.id())).to_string();

    data.characters.get(id).map(|character| {
        let builder = create_html()
            .h1(&format!("Character: {}", character.name))
            .h2("Data")
            .field_usize("Id:", id.id())
            .field("Gender:", &character.gender.to_string())
            .option(data.cultures.get(character.culture), |culture, b| {
                b.complex_field("Culture:", |b| {
                    b.link(&link_culture_details(culture.id()), culture.name().str())
                })
            })
            .p(|b| b.link(&edit_uri, "Edit"))
            .p(|b| b.link(&link_all_characters(), "Back"));

        RawHtml(builder.finish())
    })
}

fn get_edit_html(data: &RpgData, id: CharacterId, name_error: &str) -> Option<RawHtml<String>> {
    let submit_uri = uri!(update_character(id.id())).to_string();

    data.characters.get(id).map(|character| {
        let builder = create_html()
            .h1(&format!("Edit Character: {}", character.name))
            .field_usize("Id:", id.id())
            .form(&submit_uri, |b| {
                b.text_input(
                    "First Name",
                    "first_name",
                    &character.name.first().to_string(),
                )
                .error(name_error)
                .text_input(
                    "Middle Name",
                    "middle_name",
                    character.name.middle().map(|n| n.str()).unwrap_or(""),
                )
                .select(
                    "Last Name Type:",
                    "last_type",
                    &["None", "Family", "Patronymic", "Matronymic"],
                    match character.name.last() {
                        Lastname::None => "None",
                        Lastname::Family(_) => "Family",
                        Lastname::Patronymic(_) => "Patronymic",
                        Lastname::Matronymic(_) => "Matronymic",
                    },
                )
                .text_input(
                    "Last Name",
                    "last_name",
                    &character.name.last().name().map(|n| n.str()).unwrap_or(""),
                )
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

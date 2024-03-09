use crate::html::{create_html, EditorBuilder};
use crate::route::building::{delete_building_route, link_all_buildings};
use crate::route::building::{
    edit_building, get_all_buildings, get_building_details, update_building,
};
use crate::route::character::culture::{
    add_culture, edit_culture, get_all_cultures, get_culture_details, link_all_cultures,
    update_culture,
};
use crate::route::character::{
    add_character, edit_character, get_all_characters, get_character_details, link_all_characters,
    update_character,
};
use crate::route::mountain::{
    add_mountain, edit_mountain, get_all_mountains, get_mountain_details, link_all_mountains,
    update_mountain,
};
use crate::route::river::{
    add_river, edit_river, get_all_rivers, get_river_details, link_all_rivers, update_river,
};
use crate::route::street::{
    add_street, edit_street, get_all_streets, get_street_details, link_all_streets, update_street,
};
use crate::route::town::building::{add_building, get_building_creator, get_building_creator_map};
use crate::route::town::link_all_towns;
use crate::route::town::street::{
    add_street_to_town, get_street_editor, get_street_editor_map, remove_street_from_town,
    update_street_editor,
};
use crate::route::town::terrain::{
    edit_terrain_route, get_terrain_editor, get_terrain_editor_map, update_terrain_editor,
};
use crate::route::town::{
    add_town, edit_town, get_all_towns, get_town_details, get_town_map, update_town,
};
use crate::EditorData;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::{Route, State};

pub mod building;
pub mod character;
pub mod mountain;
pub mod river;
pub mod street;
pub mod town;
pub mod util;

#[get("/")]
pub fn home(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");
    let save_uri = uri!(save()).to_string();

    RawHtml(
        create_html()
            .h1("RPG Tools - Editor")
            .h2("Overview")
            .add_storage_link("Buildings:", &link_all_buildings(), &data.building_manager)
            .add_storage_link("Characters:", &link_all_characters(), &data.characters)
            .add_storage_link("Cultures:", &link_all_cultures(), &data.cultures)
            .add_storage_link("Mountains:", &link_all_mountains(), &data.mountain_manager)
            .add_storage_link("Rivers:", &link_all_rivers(), &data.river_manager)
            .add_storage_link("Streets:", &link_all_streets(), &data.street_manager)
            .add_storage_link("Towns:", &link_all_towns(), &data.town_manager)
            .p(|b| b.link(&save_uri, "Save"))
            .finish(),
    )
}

pub fn link_home() -> String {
    uri!(home()).to_string()
}

#[get("/save")]
pub fn save(state: &State<EditorData>) -> Status {
    let data = state.data.lock().expect("lock shared data");

    if let Err(e) = data.save() {
        println!("Failed to save: {}", e);
        return Status::InternalServerError;
    }

    Status::NoContent
}

pub fn get_routes() -> Vec<Route> {
    let mut routes = routes![
        home,
        save,
        get_all_mountains,
        get_mountain_details,
        add_mountain,
        edit_mountain,
        update_mountain,
        get_all_rivers,
        get_river_details,
        add_river,
        edit_river,
        update_river,
        get_all_streets,
        get_street_details,
        add_street,
        edit_street,
        update_street,
        get_all_towns,
        get_town_details,
        add_town,
        edit_town,
        update_town,
        get_town_map,
        get_terrain_editor,
        get_terrain_editor_map,
        update_terrain_editor,
        edit_terrain_route,
    ];
    routes.extend(routes![
        get_all_buildings,
        get_building_details,
        get_building_creator,
        get_building_creator_map,
        add_building,
        edit_building,
        delete_building_route,
        update_building,
        get_street_editor,
        get_street_editor_map,
        update_street_editor,
        add_street_to_town,
        remove_street_from_town,
        get_all_characters,
        get_character_details,
        add_character,
        edit_character,
        update_character,
        get_all_cultures,
        get_culture_details,
        add_culture,
        edit_culture,
        update_culture,
    ]);

    routes
}

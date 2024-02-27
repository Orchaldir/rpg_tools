use crate::html::{create_html, EditorBuilder};
use crate::route::building::link_all_buildings;
use crate::route::building::{
    edit_building, get_all_buildings, get_building_details, update_building,
};
use crate::route::mountain::{
    add_mountain, edit_mountain, get_all_mountains, get_mountain_details, update_mountain,
};
use crate::route::river::{add_river, edit_river, get_all_rivers, get_river_details, update_river};
use crate::route::street::{
    add_street, edit_street, get_all_streets, get_street_details, update_street,
};
use crate::route::town::building::{add_building, get_building_creator, get_building_creator_map};
use crate::route::town::link_all_towns;
use crate::route::town::street::{
    add_street_to_town, get_street_editor, get_street_editor_map, update_street_editor,
};
use crate::route::town::tile::{
    edit_tile, get_all_tiles, get_tile_edit_map, preview_tile, update_tile,
};
use crate::route::town::{
    add_town, edit_town, get_all_towns, get_town_details, get_town_map, update_town,
};
use crate::EditorData;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::{Route, State};

pub mod building;
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
            .add_storage_link("Mountains:", "/mountain/all", &data.mountain_manager)
            .add_storage_link("Rivers:", "/river/all", &data.river_manager)
            .add_storage_link("Streets:", "/street/all", &data.street_manager)
            .add_storage_link("Towns:", &link_all_towns(), &data.town_manager)
            .p(|b| b.link(&save_uri, "Save"))
            .finish(),
    )
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
        get_all_tiles,
        get_tile_edit_map,
        edit_tile,
        preview_tile,
        update_tile,
    ];
    routes.extend(routes![
        get_all_buildings,
        get_building_details,
        get_building_creator,
        get_building_creator_map,
        add_building,
        edit_building,
        update_building,
        get_street_editor,
        get_street_editor_map,
        update_street_editor,
        add_street_to_town,
    ]);

    routes
}

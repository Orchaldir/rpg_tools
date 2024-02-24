#[macro_use]
extern crate rocket;

use crate::html::{editor, EditorBuilder};
use crate::init::init;
use crate::route::building::{
    edit_building, get_all_buildings, get_building_details, link_all_buildings, update_building,
};
use crate::route::mountain::{
    add_mountain, edit_mountain, get_all_mountains, get_mountain_details, update_mountain,
};
use crate::route::river::{add_river, edit_river, get_all_rivers, get_river_details, update_river};
use crate::route::street::{
    add_street, edit_street, get_all_streets, get_street_details, update_street,
};
use crate::route::town::building::{add_building, get_building_creator, get_building_creator_map};
use crate::route::town::tile::{
    edit_tile, get_all_tiles, get_tile_edit_map, preview_tile, update_tile,
};
use crate::route::town::{
    add_town, edit_town, get_all_towns, get_town_details, get_town_map, link_all_towns, update_town,
};
use rocket::fs::FileServer;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::model::world::WorldData;
use rpg_tools_rendering::usecase::map::TileMapRenderer;
use std::sync::Mutex;

mod html;
mod init;
mod route;
mod svg;

pub struct EditorData {
    data: Mutex<WorldData>,
    town_renderer: TileMapRenderer,
}

#[get("/")]
fn hello(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");

    RawHtml(
        editor()
            .h1("RPG Tools - Editor")
            .h2("Overview")
            .add_storage_link("Buildings:", &link_all_buildings(), &data.building_manager)
            .add_storage_link("Mountains:", "/mountain/all", &data.mountain_manager)
            .add_storage_link("Rivers:", "/river/all", &data.river_manager)
            .add_storage_link("Streets:", "/street/all", &data.street_manager)
            .add_storage_link("Towns:", &link_all_towns(), &data.town_manager)
            .finish(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(EditorData {
            data: Mutex::new(init()),
            town_renderer: TileMapRenderer::new(100, 10, 1),
        })
        .mount("/static", FileServer::from("rpg_tools_editor/static/"))
        .mount(
            "/",
            routes![
                hello,
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
                get_all_buildings,
                get_building_details,
                get_building_creator,
                get_building_creator_map,
                add_building,
                edit_building,
                update_building
            ],
        )
}

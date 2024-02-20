#[macro_use]
extern crate rocket;

use crate::init::init;
use crate::route::mountain::{
    add_mountain, edit_mountain, get_all_mountains, get_mountain_details, update_mountain,
};
use crate::route::river::{add_river, edit_river, get_all_rivers, get_river_details, update_river};
use crate::route::street::{
    add_street, edit_street, get_all_streets, get_street_details, update_street,
};
use crate::route::town::terrain::{
    edit_terrain, get_all_terrain, get_terrain_edit_map, update_tile,
};
use crate::route::town::{
    add_town, edit_town, get_all_towns, get_town_details, get_town_map, update_town,
};
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::WorldData;
use rpg_tools_rendering::usecase::map::EdgeMapRenderer;
use std::sync::Mutex;

mod init;
mod route;
mod svg;

pub struct EditorData {
    data: Mutex<WorldData>,
    town_renderer: EdgeMapRenderer,
}

#[get("/")]
fn hello(state: &State<EditorData>) -> Template {
    let data = state.data.lock().expect("lock shared data");

    Template::render(
        "home",
        context! {
            mountains: data.mountain_manager.get_all().len(),
            rivers: data.river_manager.get_all().len(),
            streets: data.street_manager.get_all().len(),
            towns: data.town_manager.get_all().len(),
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(EditorData {
            data: Mutex::new(init()),
            town_renderer: EdgeMapRenderer::new(100, 10, 1),
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
                get_all_terrain,
                edit_terrain,
                get_terrain_edit_map,
                update_tile,
            ],
        )
        .attach(Template::fairing())
}

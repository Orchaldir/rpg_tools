#[macro_use]
extern crate rocket;

use crate::init::init;
use crate::route::mountain::{get_all_mountains, get_mountain_details};
use crate::route::river::get_all_rivers;
use crate::route::street::get_all_streets;
use crate::route::town::get_all_towns;
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::WorldData;

mod init;
mod route;

pub struct EditorData {
    data: WorldData,
}

#[get("/")]
fn hello(data: &State<EditorData>) -> Template {
    Template::render(
        "home",
        context! {
            mountains: data.data.mountain_manager.get_all().len(),
            rivers: data.data.river_manager.get_all().len(),
            streets: data.data.street_manager.get_all().len(),
            towns: data.data.town_manager.get_all().len(),
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(EditorData { data: init() })
        .mount("/static", FileServer::from("rpg_tools_editor/static/"))
        .mount(
            "/",
            routes![
                hello,
                get_all_mountains,
                get_mountain_details,
                get_all_rivers,
                get_all_streets,
                get_all_towns
            ],
        )
        .attach(Template::fairing())
}

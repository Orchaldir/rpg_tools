#[macro_use]
extern crate rocket;

use crate::init::init;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::world::WorldData;

mod init;

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
        .mount("/", routes![hello])
        .attach(Template::fairing())
}

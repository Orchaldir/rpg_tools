#[macro_use]
extern crate rocket;

use crate::init::init;
use rpg_tools_core::model::world::WorldData;

mod init;

pub struct EditorData {
    data: WorldData,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(EditorData { data: init() })
        .mount("/", routes![hello])
}

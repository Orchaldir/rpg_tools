#[macro_use]
extern crate rocket;

use crate::init::init;
use crate::route::get_routes;
use rocket::fs::FileServer;
use rpg_tools_core::model::world::street::StreetId;
use rpg_tools_core::model::WorldData;
use rpg_tools_rendering::usecase::map::TileMapRenderer;
use std::sync::Mutex;

mod html;
mod init;
mod route;
mod svg;

pub struct ToolData {
    selected_street: StreetId,
    terrain: String,
    id: Option<usize>,
}

pub struct EditorData {
    data: Mutex<WorldData>,
    town_renderer: TileMapRenderer,
    tools: Mutex<ToolData>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(EditorData {
            data: Mutex::new(init().unwrap()),
            town_renderer: TileMapRenderer::new(100, 1),
            tools: Mutex::new(ToolData {
                selected_street: StreetId::default(),
                terrain: "Plain".to_string(),
                id: None,
            }),
        })
        .mount("/static", FileServer::from("rpg_tools_editor/static/"))
        .mount("/", get_routes())
}

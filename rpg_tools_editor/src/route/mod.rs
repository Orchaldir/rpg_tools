use crate::html::{create_html, EditorBuilder};
use crate::route::building::link_all_buildings;
use crate::route::town::link_all_towns;
use crate::EditorData;
use rocket::response::content::RawHtml;
use rocket::State;
use rpg_tools_core::utils::storage::{Element, Id};

pub mod building;
pub mod mountain;
pub mod river;
pub mod street;
pub mod town;
pub mod util;

#[get("/")]
pub fn home(state: &State<EditorData>) -> RawHtml<String> {
    let data = state.data.lock().expect("lock shared data");

    RawHtml(
        create_html()
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

use crate::model::world::town::towns::WithTowns;
use crate::model::world::town::TownId;
use crate::utils::storage::{Element, Id, Storage};

pub fn contains_town<I: Id, T: Element<I> + WithTowns>(
    storage: &Storage<I, T>,
    id: I,
    town_id: TownId,
) -> bool {
    storage
        .get(id)
        .map(|e| e.towns().contains(&town_id))
        .unwrap_or(false)
}

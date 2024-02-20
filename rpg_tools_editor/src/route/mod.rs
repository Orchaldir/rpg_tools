use rocket_dyn_templates::{context, Template};
use rpg_tools_core::utils::storage::{Element, Id, Storage};
use std::collections::HashSet;

pub mod mountain;
pub mod river;
pub mod street;
pub mod town;

pub fn get_all_elements<ID: Id, ELEMENT: Element<ID>>(
    storage: &Storage<ID, ELEMENT>,
) -> Vec<(usize, &str)> {
    storage
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name()))
        .collect()
}

pub fn get_elements<'a, ID: Id, ELEMENT: Element<ID>>(
    storage: &'a Storage<ID, ELEMENT>,
    ids: &'a HashSet<ID>,
) -> Vec<(usize, &'a str)> {
    ids.iter()
        .flat_map(|id| storage.get(*id))
        .map(|c| (c.id().id(), c.name()))
        .collect()
}

pub fn get_all_template<ID: Id, ELEMENT: Element<ID>>(
    storage: &Storage<ID, ELEMENT>,
    name: &str,
    title: &str,
) -> Template {
    Template::render(
        "generic/all",
        context! {
            name: name,
            title: title,
            values: get_all_elements(storage),
        },
    )
}

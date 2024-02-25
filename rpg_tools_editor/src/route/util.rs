use crate::html::create_html;
use rocket::response::content::RawHtml;
use rpg_tools_core::utils::storage::{Element, Id, Storage};
use std::collections::HashSet;

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
) -> Vec<&'a ELEMENT> {
    ids.iter().flat_map(|id| storage.get(*id)).collect()
}

pub fn get_all_html<ID: Id, ELEMENT: Element<ID>>(
    storage: &Storage<ID, ELEMENT>,
    name: &str,
    title: &str,
) -> RawHtml<String> {
    RawHtml(
        create_html()
            .h1(title)
            .field("Count:", &storage.get_all().len().to_string())
            .list(storage.get_all(), |b, e| {
                b.link(&format!("/{}/{}/details", name, e.id().id()), e.name())
            })
            .p(|b| b.link(&format!("/{}/new", name), "Add"))
            .p(|b| b.link("/", "Back"))
            .finish(),
    )
}
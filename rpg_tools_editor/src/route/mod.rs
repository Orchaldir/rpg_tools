use rocket_dyn_templates::{context, Template};
use rpg_tools_core::utils::storage::{Element, Id, Storage};

pub mod mountain;
pub mod river;
pub mod street;
pub mod town;

pub fn get_all_template<ID: Id, ELEMENT: Element<ID>>(
    storage: &Storage<ID, ELEMENT>,
    name: &str,
    title: &str,
) -> Template {
    let values: Vec<(usize, &str)> = storage
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name()))
        .collect();

    Template::render(
        "generic/all",
        context! {
            name: name,
            title: title,
            values: values,
        },
    )
}

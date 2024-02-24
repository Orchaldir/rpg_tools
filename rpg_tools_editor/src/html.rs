use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_core::utils::storage::{Element, Id, Storage};
use rpg_tools_html::HtmlBuilder;

pub fn create_html() -> HtmlBuilder {
    HtmlBuilder::new("RPG Tools - Editor")
}

pub trait EditorBuilder {
    fn field_size2d(self, name: &str, size: &Size2d) -> Self;

    fn add_storage_link<ID: Id, ELEMENT: Element<ID>>(
        self,
        title: &str,
        link: &str,
        storage: &Storage<ID, ELEMENT>,
    ) -> Self;
}

impl EditorBuilder for HtmlBuilder {
    fn field_size2d(self, name: &str, size: &Size2d) -> Self {
        self.complex_field(name, |b| {
            b.text(&format!("{} x {}", size.width(), size.height()))
        })
    }

    fn add_storage_link<ID: Id, ELEMENT: Element<ID>>(
        self,
        title: &str,
        link: &str,
        storage: &Storage<ID, ELEMENT>,
    ) -> Self {
        self.p(|builder| {
            builder
                .bold(title)
                .complex_link(link, |a| a.usize(storage.get_all().len()))
        })
    }
}

use crate::html::HtmlBuilder;

pub struct FormBuilder {
    html: HtmlBuilder,
}

impl FormBuilder {
    pub fn new(html: HtmlBuilder) -> Self {
        Self { html }
    }

    pub fn finish(mut self) -> HtmlBuilder {
        self.html
    }
}

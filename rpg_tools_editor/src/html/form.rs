use crate::html::HtmlBuilder;

pub struct FormBuilder {
    html: HtmlBuilder,
}

impl FormBuilder {
    pub fn new(html: HtmlBuilder) -> Self {
        Self { html }
    }

    pub fn text_input(self, label: &str, name: &str, value: &str) -> Self {
        Self {
            html: self.html.p(|b| {
                b.open_tag_with_attribute("label", "for", "name")
                    .bold(label)
                    .close_tag()
                    .text(&format!(
                        r#"<input type="text" id="{0}" name="{0}" value="{1}">"#,
                        name, value
                    ))
            }),
        }
    }

    pub fn error(self, error: &str) -> Self {
        if error.is_empty() {
            self
        } else {
            Self {
                html: self.html.p(|b| {
                    b.open_tag_with_attribute("label", "class", "error")
                        .text(error)
                        .close_tag()
                }),
            }
        }
    }

    pub fn finish(self) -> HtmlBuilder {
        self.html
    }
}

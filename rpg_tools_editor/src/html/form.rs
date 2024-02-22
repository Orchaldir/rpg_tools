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
                b.open_tag_with_attribute("label", "for", name)
                    .bold(label)
                    .close_tag()
                    .text(&format!(
                        r#"<input type="text" id="{0}" name="{0}" value="{1}">"#,
                        name, value
                    ))
            }),
        }
    }

    pub fn number_input(
        self,
        label: &str,
        name: &str,
        value: usize,
        min: usize,
        max: usize,
    ) -> Self {
        Self {
            html: self.html.p(|b| {
                b.open_tag_with_attribute("label", "for", name)
                    .bold(label)
                    .close_tag()
                    .text(&format!(
                        r#"<input type="number" id="{0}" name="{0}" value="{1}" step="1" min="{2}" max="{3}">"#,
                        name, value, min, max
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

    pub fn select(self, label: &str, name: &str, values: &[&str], selected: &str) -> Self {
        Self {
            html: self.html.p(|mut b| {
                b = b
                    .open_tag_with_attribute("label", "for", name)
                    .bold(label)
                    .close_tag()
                    .open_tag_with_3_attributes(
                        "select",
                        "id",
                        name,
                        "name",
                        name,
                        "onchange",
                        "updateEditor();",
                    );

                for &value in values {
                    if value.eq(selected) {
                        b.add(format!(
                            r#"<option value="{0}" selected>{0}</option>"#,
                            value
                        ));
                    } else {
                        b.add(format!(r#"<option value="{0}">{0}</option>"#, value));
                    }
                }

                b.close_tag()
            }),
        }
    }

    pub fn select_id(
        self,
        label: &str,
        name: &str,
        values: &[(usize, &str)],
        selected: usize,
    ) -> Self {
        Self {
            html: self.html.p(|mut b| {
                b = b
                    .open_tag_with_attribute("label", "for", name)
                    .bold(label)
                    .close_tag()
                    .open_tag_with_3_attributes(
                        "select",
                        "id",
                        name,
                        "name",
                        name,
                        "onchange",
                        "updateEditor();",
                    );

                for &(id, value) in values {
                    if id == selected {
                        b.add(format!(
                            r#"<option value="{}" selected>{}</option>"#,
                            id, value
                        ));
                    } else {
                        b.add(format!(r#"<option value="{}">{}</option>"#, id, value));
                    }
                }

                b.close_tag()
            }),
        }
    }

    pub fn finish(self) -> HtmlBuilder {
        self.html
    }
}

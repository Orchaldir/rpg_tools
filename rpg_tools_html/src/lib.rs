pub mod form;

use crate::form::FormBuilder;

pub struct HtmlBuilder {
    lines: Vec<String>,
    elements: Vec<String>,
}

impl HtmlBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            lines: vec!["<!DOCTYPE html>".to_string()],
            elements: Vec::new(),
        }
        .open_tag("html")
        .open_tag("head")
        .text(r#"<link rel="stylesheet" href="/static/style.css">"#)
        .text(r#"<script src="/static/scripts.js" charset="utf-8" defer></script>"#)
        .inline_tag("title", title)
        .close_tag()
        .open_tag("body")
    }

    pub fn finish(mut self) -> String {
        while !self.elements.is_empty() {
            self = self.close_tag();
        }

        self.lines.join("\n")
    }

    fn add(&mut self, line: String) {
        self.lines.push(format!("{}{}", self.indent(), line));
    }

    fn add_tag_with_2_attributes(
        &mut self,
        tag: &str,
        attribute0: &str,
        value0: &str,
        attribute1: &str,
        value1: &str,
    ) {
        self.add(format!(
            r#"<{} {}="{}" {}="{}">"#,
            tag, attribute0, value0, attribute1, value1
        ));
    }

    fn add_tag_with_3_attributes(
        &mut self,
        tag: &str,
        attribute0: &str,
        value0: &str,
        attribute1: &str,
        value1: &str,
        attribute2: &str,
        value2: &str,
    ) {
        self.add(format!(
            r#"<{} {}="{}" {}="{}" {}="{}">"#,
            tag, attribute0, value0, attribute1, value1, attribute2, value2
        ));
    }

    fn indent(&self) -> String {
        "  ".repeat(self.elements.len())
    }

    fn open_tag(mut self, tag: &str) -> Self {
        self.add(format!("<{}>", tag));
        self.elements.push(tag.to_string());
        self
    }

    fn open_tag_with_attribute(mut self, tag: &str, attribute: &str, value: &str) -> Self {
        self.add(format!(r#"<{} {}="{}">"#, tag, attribute, value));
        self.elements.push(tag.to_string());
        self
    }

    fn open_tag_with_2_attributes(
        mut self,
        tag: &str,
        attribute0: &str,
        value0: &str,
        attribute1: &str,
        value1: &str,
    ) -> Self {
        self.add_tag_with_2_attributes(tag, attribute0, value0, attribute1, value1);
        self.elements.push(tag.to_string());
        self
    }

    fn open_tag_with_3_attributes(
        mut self,
        tag: &str,
        attribute0: &str,
        value0: &str,
        attribute1: &str,
        value1: &str,
        attribute2: &str,
        value2: &str,
    ) -> Self {
        self.add_tag_with_3_attributes(
            tag, attribute0, value0, attribute1, value1, attribute2, value2,
        );
        self.elements.push(tag.to_string());
        self
    }

    fn close_tag(mut self) -> Self {
        if let Some(element) = self.elements.pop() {
            self.add(format!("</{}>", element));
        }
        self
    }

    fn inline_tag(mut self, tag: &str, value: &str) -> Self {
        self.add(format!("<{0}>{1}</{0}>", tag, value));
        self
    }

    fn tag<F: FnOnce(Self) -> Self>(mut self, tag: &str, f: F) -> Self {
        self = self.open_tag(tag);

        self = f(self);

        self.close_tag()
    }

    pub fn h1(self, title: &str) -> Self {
        self.inline_tag("h1", title)
    }

    pub fn h2(self, title: &str) -> Self {
        self.inline_tag("h2", title)
    }

    pub fn h3(self, title: &str) -> Self {
        self.inline_tag("h3", title)
    }

    pub fn p<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        self.tag("p", f)
    }

    pub fn center<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        self.tag("center", f)
    }

    pub fn link(self, link: &str, text: &str) -> Self {
        self.complex_link(link, |b| b.text(text))
    }

    pub fn complex_link<F: FnOnce(Self) -> Self>(mut self, link: &str, f: F) -> Self {
        self = self.open_tag_with_attribute("a", "href", link);

        self = f(self);

        self.close_tag()
    }

    pub fn list<T, F: FnMut(Self, &T) -> Self>(mut self, list: &[T], mut f: F) -> Self {
        self = self.open_tag("ul");

        for element in list {
            self = self.open_tag("li");
            self = f(self, element);
            self = self.close_tag();
        }

        self.close_tag()
    }

    pub fn bold(self, text: &str) -> Self {
        self.inline_tag("b", text)
    }

    pub fn text(mut self, text: &str) -> Self {
        self.add(text.to_string());
        self
    }

    pub fn usize(self, number: usize) -> Self {
        self.text(&number.to_string())
    }

    pub fn form<F: FnOnce(FormBuilder) -> FormBuilder>(mut self, submit: &str, f: F) -> Self {
        self = self.open_tag("form");

        Self::submit_button(f(FormBuilder::new(self)), submit)
    }

    pub fn form_with_change<F: FnOnce(FormBuilder) -> FormBuilder>(
        mut self,
        update: &str,
        submit: &str,
        f: F,
    ) -> Self {
        self = self
            .open_tag_with_3_attributes("form", "id", "editor", "action", update, "method", "post");

        Self::submit_button(f(FormBuilder::new(self)), submit)
    }

    fn submit_button(builder: FormBuilder, submit: &str) -> Self {
        builder
            .finish()
            .open_tag_with_2_attributes("button", "formaction", submit, "formmethod", "post")
            .text("Submit")
            .close_tag()
            .close_tag()
    }

    pub fn image(mut self, source: &str, text: &str, width: &str) -> Self {
        self.add_tag_with_3_attributes("img", "src", source, "alt", text, "width", width);
        self
    }

    pub fn svg(self, source: &str, width: &str) -> Self {
        self.open_tag_with_3_attributes(
            "object",
            "data",
            source,
            "type",
            "image/svg+xml",
            "width",
            width,
        )
        .close_tag()
    }

    pub fn complex_field<F: FnOnce(Self) -> Self>(self, name: &str, f: F) -> Self {
        self.p(|builder| f(builder.bold(name)))
    }

    pub fn field(self, name: &str, value: &str) -> Self {
        self.complex_field(name, |b| b.text(value))
    }

    pub fn field_usize(self, name: &str, value: usize) -> Self {
        self.complex_field(name, |b| b.usize(value))
    }

    /// Only executes *f*, if *option* is some value.
    pub fn option<T, F: FnOnce(T, Self) -> Self>(self, option: Option<T>, f: F) -> Self {
        if let Some(value) = option {
            f(value, self)
        } else {
            self
        }
    }
}

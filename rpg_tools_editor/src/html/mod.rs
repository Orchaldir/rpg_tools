pub struct HtmlBuilder {
    lines: Vec<String>,
    elements: Vec<String>,
}

impl HtmlBuilder {
    pub fn new() -> Self {
        let mut lines = Vec::new();

        lines.push("<!DOCTYPE html>".to_string());

        Self {
            lines,
            elements: Vec::new(),
        }
        .open_tag("html")
        .open_tag("head")
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

    pub fn h1(self, title: &str) -> Self {
        self.inline_tag("h1", title)
    }

    pub fn h2(self, title: &str) -> Self {
        self.inline_tag("h2", title)
    }

    pub fn p<F: FnOnce(Self) -> Self>(mut self, f: F) -> Self {
        self = self.open_tag("p");

        self = f(self);

        self.close_tag()
    }

    pub fn a<F: FnOnce(Self) -> Self>(mut self, link: &str, f: F) -> Self {
        self = self.open_tag_with_attribute("a", "href", link);

        self = f(self);

        self.close_tag()
    }

    pub fn b(self, text: &str) -> Self {
        self.inline_tag("b", text)
    }

    pub fn text(mut self, text: &str) -> Self {
        self.add(text.to_string());
        self
    }

    pub fn usize(self, number: usize) -> Self {
        self.text(&number.to_string())
    }
}

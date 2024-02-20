use rpg_tools_core::model::math::size2d::Size2d;
use rpg_tools_rendering::renderer::svg::Svg;

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

        self.lines.push("</svg>".to_string());

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
        self.elements.push("tag".to_string());
        self
    }

    fn close_tag(mut self) -> Self {
        if let Some(element) = self.elements.pop() {
            self.add(format!("</{}>", element));
        }
        self
    }

    pub fn h1(mut self, title: &str) -> Self {
        self.add(format!("<h1>{}</h1>", title));
        self
    }
}

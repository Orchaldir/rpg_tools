use crate::renderer::style::RenderStyle;
use crate::renderer::svg::Svg;
use crate::renderer::Renderer;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::math::aabb2d::AABB;
use rpg_tools_core::model::math::point2d::Point2d;
use rpg_tools_core::model::math::size2d::Size2d;

/// Builds a valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Debug, PartialEq, Eq)]
pub struct SvgBuilder {
    lines: Vec<String>,
    elements: Vec<String>,
}

impl SvgBuilder {
    pub fn new(size: Size2d) -> Self {
        let mut lines = Vec::new();

        lines.push(format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            size.width(),
            size.height()
        ));

        Self {
            lines,
            elements: Vec::new(),
        }
    }

    fn add(&mut self, line: String) {
        self.lines.push(format!("{}{}", self.indent(), line));
    }

    fn indent(&self) -> String {
        "  ".repeat(self.elements.len() + 1)
    }

    pub fn link(&mut self, link: &str) {
        self.add(format!("<a href=\"{}\">", link));
        self.elements.push("a".to_string());
    }

    pub fn close(&mut self) {
        if let Some(element) = self.elements.pop() {
            self.add(format!("</{}>", element));
        }
    }

    pub fn finish(mut self) -> Svg {
        while !self.elements.is_empty() {
            self.close();
        }

        self.lines.push("</svg>".to_string());

        Svg { lines: self.lines }
    }
}

impl Renderer for SvgBuilder {
    fn render_circle(&mut self, center: &Point2d, radius: u32, style: &RenderStyle) {
        self.add(format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" style=\"{}\"/>",
            center.x,
            center.y,
            radius,
            to_style(style),
        ));
    }

    fn render_rectangle(&mut self, aabb: &AABB, style: &RenderStyle) {
        self.add(format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"{}\"/>",
            aabb.start().x,
            aabb.start().y,
            aabb.size().width(),
            aabb.size().height(),
            to_style(style),
        ));
    }
}

fn to_style(style: &RenderStyle) -> String {
    match style {
        RenderStyle::NoBorder(color) => to_color(color, "fill"),
        RenderStyle::OnlyBorder {
            border_color,
            border_width,
        } => format!("fill:none;{}", to_stroke(border_color, *border_width)),
        RenderStyle::WithBorder {
            fill_color,
            border_color,
            border_width,
        } => format!(
            "{};{}",
            to_color(fill_color, "fill"),
            to_stroke(border_color, *border_width)
        ),
    }
}

fn to_stroke(color: &Color, width: u32) -> String {
    format!("{};stroke-width:{}", to_color(color, "stroke"), width)
}

fn to_color(color: &Color, text: &str) -> String {
    format!("{}:{}", text, color.to_string().to_lowercase())
}

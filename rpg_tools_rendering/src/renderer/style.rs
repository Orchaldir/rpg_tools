use rpg_tools_core::model::color::Color;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderStyle {
    NoBorder(Color),
    OnlyBorder {
        border_color: Color,
        border_width: u32,
    },
    WithBorder {
        fill_color: Color,
        border_color: Color,
        border_width: u32,
    },
}

impl RenderStyle {
    pub fn no_border(color: Color) -> Self {
        Self::NoBorder(color)
    }

    pub fn only_border(color: Color, width: u32) -> Self {
        Self::OnlyBorder {
            border_color: color,
            border_width: width,
        }
    }

    pub const fn with_border(fill_color: Color, border_color: Color, border_width: u32) -> Self {
        Self::WithBorder {
            fill_color,
            border_color,
            border_width,
        }
    }
}

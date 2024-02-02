/// A color defined by a name.
/// See https://en.wikipedia.org/wiki/Web_colors.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Aqua,
    Black,
    Blue,
    Fuchsia,
    Gray,
    Green,
    Lime,
    Maroon,
    Navy,
    Olive,
    Orange,
    #[default]
    Purple,
    Red,
    SaddleBrown,
    Silver,
    Teal,
    White,
    Yellow,
}

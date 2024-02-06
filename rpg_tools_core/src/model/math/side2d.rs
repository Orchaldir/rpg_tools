use core::fmt;

/// The 4 sides of a rectangle.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Side2d {
    Top,
    Left,
    Bottom,
    Right,
}

impl fmt::Display for Side2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use std::ops::{Div, Mul, Sub};

/// Defines the size of a 2d rectangle.
///
/// # Diagram
///
/// ```svgbob
///       0   1
///   +----------> x-axis
///   |
///   | +---+---+
/// 0 | |       |
///   | +       +
/// 1 | |       |
///   | +       +
/// 2 | |       |
///   | +---+---+
///   v
/// y-axis
/// ```
///
/// The min size for any axis is 1.
///
/// ```
///# use rpg_tools_core::model::math::size2d::Size2d;
/// assert_eq!(Size2d::new(2, 0), Size2d::new(2, 1));
/// assert_eq!(Size2d::new(0, 3), Size2d::new(1, 3));
/// ```

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Size2d {
    width: i32,
    height: i32,
}

impl Size2d {
    /// Returns a new size.
    pub fn new(width: u32, height: u32) -> Self {
        Size2d {
            width: width.max(1) as i32,
            height: height.max(1) as i32,
        }
    }

    /// Returns a size with equal width & height.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// assert_eq!(Size2d::square(2), Size2d::new(2, 2));
    /// ```
    pub fn square(size: u32) -> Self {
        Size2d::new(size, size)
    }

    /// Returns the size along the x-axis.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.width(), 2);
    /// ```
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Returns the size along the y-axis.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.height(), 3);
    /// ```
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Returns the number of tiles covered by this size.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.tiles(), 6);
    /// ```
    pub fn tiles(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Is the point inside?
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    ///
    /// assert!(size.is_inside(1, 2));
    /// assert!(!size.is_inside(4, 5));
    /// ```
    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// Converts an index to the x-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_x(5), 1);
    /// ```
    pub fn to_x(&self, index: usize) -> i32 {
        index as i32 % self.width
    }

    /// Converts an index to the y-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_y(5), 2);
    /// ```
    pub fn to_y(&self, index: usize) -> i32 {
        index as i32 / self.width
    }

    /// Converts a point to the equivalent index, if it is inside.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    ///
    /// assert_eq!(size.to_index(1, 2), Some(5));
    /// assert_eq!(size.to_index(4, 5), None);
    /// assert_eq!(size.to_index(-1, 0), None);
    /// assert_eq!(size.to_index(0, -1), None);
    /// ```
    pub fn to_index(&self, x: i32, y: i32) -> Option<usize> {
        if self.is_inside(x, y) {
            return Some(self.to_index_risky(x, y));
        }

        None
    }

    /// Converts a point to the equivalent index, but returns a wrong result if it is outside.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_index_risky(1, 2), 5);
    /// ```
    pub fn to_index_risky(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Scales the size.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// assert_eq!(Size2d::new(10, 30).scale(0.2, 0.5), Size2d::new(2, 15));
    /// ```
    pub fn scale(&self, horizontal: f32, vertical: f32) -> Self {
        Size2d::new(
            (self.width as f32 * horizontal) as u32,
            (self.height as f32 * vertical) as u32,
        )
    }
}

impl Sub<Size2d> for Size2d {
    type Output = Self;

    /// Subtracts a size from another.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// assert_eq!(Size2d::new(10, 30) - Size2d::new(2, 5), Size2d::new(8, 25));
    /// ```
    fn sub(self, other: Size2d) -> Self::Output {
        Size2d::new(
            (self.width - other.width()) as u32,
            (self.height - other.height) as u32,
        )
    }
}

impl Div<f32> for Size2d {
    type Output = Self;

    /// Divides a size by a f32.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// assert_eq!(Size2d::new(10, 30) / 0.5, Size2d::new(20, 60));
    /// ```
    fn div(self, value: f32) -> Self::Output {
        Size2d::new(
            (self.width as f32 / value) as u32,
            (self.height as f32 / value) as u32,
        )
    }
}

impl Mul<f32> for Size2d {
    type Output = Self;

    /// Multiplies a size by a f32.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// assert_eq!(Size2d::new(10, 30) * 1.5, Size2d::new(15, 45));
    /// ```
    fn mul(self, value: f32) -> Self::Output {
        Size2d::new(
            (self.width as f32 * value) as u32,
            (self.height as f32 * value) as u32,
        )
    }
}

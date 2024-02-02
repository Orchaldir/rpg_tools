use std::ops::{Div, Mul};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Size2d {
    width: u32,
    height: u32,
}

impl Size2d {
    /// Returns a new size.
    pub const fn new(width: u32, height: u32) -> Self {
        Size2d { width, height }
    }

    /// Returns a size with equal width & height.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// assert_eq!(Size2d::square(2), Size2d::new(2, 2));
    /// ```
    pub const fn square(size: u32) -> Self {
        Size2d::new(size, size)
    }

    /// Returns the size along the x-axis.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.width(), 2);
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the size along the y-axis.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.height(), 3);
    /// ```
    pub fn height(&self) -> u32 {
        self.height
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

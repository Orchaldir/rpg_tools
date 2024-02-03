use crate::model::math::size2d::Size2d;
use std::ops::{Add, Div, Mul, Sub};

/// Defines a point in 2 dimensions.
///
/// # Diagram
///
/// ```svgbob
///      0  1
///   +--*--*----> x-axis
///   |
/// 0 *
///   |
/// 1 *
///   |
/// 2 *     * (1,2)
///   |
///   v
/// y-axis
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    /// Returns a new point.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let point = Point2d::new(2, 3);
    /// assert_eq!(point.x, 2);
    /// assert_eq!(point.y, 3);
    /// ```
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns a new point on the y-axis.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// assert_eq!(Point2d::vertical(3), Point2d::new(0, 3));
    /// ```
    pub const fn vertical(y: i32) -> Self {
        Self { x: 0, y }
    }

    /// Calculates the euclidean distance to another point.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let a = Point2d::new(1, 2);
    /// let b = Point2d::new(4, 6);
    ///
    /// assert_eq!(a.calculate_distance(&a), 0.0);
    /// assert_eq!(a.calculate_distance(&b), 5.0);
    /// assert_eq!(b.calculate_distance(&a), 5.0);
    /// ```
    pub fn calculate_distance(&self, point: &Point2d) -> f32 {
        (self.x as f32 - point.x as f32).hypot(self.y as f32 - point.y as f32)
    }
}

impl Add<Point2d> for Point2d {
    type Output = Point2d;

    /// Adds 2 points together.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let a = Point2d::new(1, 2);
    /// let b = Point2d::new(30, 50);
    /// let result = Point2d::new(31, 52);
    ///
    /// assert_eq!(a + b, result);
    /// assert_eq!(b + a, result);
    /// ```
    fn add(self, other: Point2d) -> Point2d {
        Point2d::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<Size2d> for Point2d {
    type Output = Point2d;

    /// Adds a [`Size2d`] to a point.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let a = Point2d::new(1, 2);
    /// let b = Size2d::new(30, 50);
    /// let result = Point2d::new(31, 52);
    ///
    /// assert_eq!(a + b, result);
    /// ```
    fn add(self, other: Size2d) -> Point2d {
        Point2d::new(
            self.x + other.width() as i32,
            self.y + other.height() as i32,
        )
    }
}

impl Div<f32> for Point2d {
    type Output = Self;

    /// Divides a point by a f32.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// assert_eq!(Point2d::new(10, 30) / 0.5, Point2d::new(20, 60));
    /// ```
    fn div(self, value: f32) -> Self::Output {
        Point2d::new(
            (self.x as f32 / value) as i32,
            (self.y as f32 / value) as i32,
        )
    }
}

impl Mul<f32> for Point2d {
    type Output = Self;

    /// Multiplies a point by a f32.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// assert_eq!(Point2d::new(10, 30) * 1.5, Point2d::new(15, 45));
    /// ```
    fn mul(self, value: f32) -> Self::Output {
        Point2d::new(
            (self.x as f32 * value) as i32,
            (self.y as f32 * value) as i32,
        )
    }
}

impl Sub<Point2d> for Point2d {
    type Output = Point2d;

    /// Subtracts a point from another point.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let a = Point2d::new(1, 2);
    /// let b = Point2d::new(30, 50);
    ///
    /// assert_eq!(a - b, Point2d::new(-29, -48));
    /// assert_eq!(b - a, Point2d::new(29, 48));
    /// ```
    fn sub(self, other: Point2d) -> Point2d {
        Point2d::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Size2d> for Point2d {
    type Output = Point2d;

    /// Subtracts a [`Size2d`] from a point.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let a = Point2d::new(1, 2);
    /// let b = Size2d::new(30, 50);
    /// let result = Point2d::new(-29, -48);
    ///
    /// assert_eq!(a - b, result);
    /// ```
    fn sub(self, other: Size2d) -> Point2d {
        Point2d::new(
            self.x - other.width() as i32,
            self.y - other.height() as i32,
        )
    }
}

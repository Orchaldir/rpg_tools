use crate::model::math::point2d::Point2d;
use crate::model::math::size2d::Size2d;

pub type AABB = AxisAlignedBoundingBox;

/// Defines a 2d axis aligned bounding box.
///
/// # Diagram
///
/// ```svgbob
///   +---------------------> x-axis
///   |     start
///   |     *---------*
///   |     |         |
///   |     |         |
///   |     |         |
///   |     *---------*
///   |           end = start + size
///   v
/// y-axis
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AxisAlignedBoundingBox {
    start: Point2d,
    end: Point2d,
    size: Size2d,
}

impl AxisAlignedBoundingBox {
    /// Returns a new axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AABB;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let start = Point2d::new(2, 3);
    /// let size = Size2d::new(30, 50);
    /// let aabb = AABB::new(start, size);
    ///
    /// assert_eq!(aabb.start(), &start);
    /// assert_eq!(aabb.end(), &Point2d::new(32, 53));
    /// assert_eq!(aabb.size(), &size);
    /// ```
    pub fn new(start: Point2d, size: Size2d) -> Self {
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    /// Returns a new axis aligned bounding box initialized with primitives.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AABB;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let aabb = AABB::simple(2, 3, 30, 50);
    ///
    /// assert_eq!(aabb.start(), &Point2d::new(2, 3));
    /// assert_eq!(aabb.end(), &Point2d::new(32, 53));
    /// assert_eq!(aabb.size(), &Size2d::new(30, 50));
    /// ```
    pub fn simple(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self::new(Point2d::new(x, y), Size2d::new(width, height))
    }

    /// Returns a new axis aligned bounding box starting at the origin.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AABB;
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// assert_eq!(AABB::with_size(Size2d::new(30, 50)), AABB::simple(0, 0, 30, 50));
    /// ```
    pub fn with_size(size: Size2d) -> Self {
        let start = Point2d::new(0, 0);
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    /// Returns a new axis aligned bounding box around a center.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AABB;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let center = Point2d::new(100, 200);
    /// let size = Size2d::new(30, 50);
    /// let start = Point2d::new(85, 175);
    /// assert_eq!(AABB::with_center(center, size), AABB::new(start, size));
    /// ```
    pub fn with_center(center: Point2d, size: Size2d) -> Self {
        let start = center - size / 2.0;
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    /// Returns a new axis aligned bounding box from a center & radii.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AABB;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    ///# use rpg_tools_core::model::math::size2d::Size2d;
    /// let center = Point2d::new(100, 200);
    /// let size = Size2d::new(20, 40);
    /// let start = Point2d::new(90, 180);
    /// assert_eq!(AABB::with_radii(center, 10, 20), AABB::new(start, size));
    /// ```
    pub fn with_radii(center: Point2d, radius_x: u32, radius_y: u32) -> Self {
        let size = Size2d::new(radius_x, radius_y) * 2.0;
        AABB::with_center(center, size)
    }

    pub fn start(&self) -> &Point2d {
        &self.start
    }

    /// Returns the center of the axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(2, 3, 30, 50);
    ///
    /// assert_eq!(aabb.center(), Point2d::new(17, 28));
    /// ```
    pub fn center(&self) -> Point2d {
        self.start + self.size / 2.0
    }

    /// Returns the 4 corners of the axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(2, 3, 30, 50);
    ///
    /// assert_eq!(aabb.corners(), vec![Point2d::new(2, 3), Point2d::new(32, 3), Point2d::new(32, 53), Point2d::new(2, 53)]);
    /// ```
    pub fn corners(&self) -> Vec<Point2d> {
        vec![
            self.start,
            self.get_point(1.0, 0.0),
            self.end,
            self.get_point(0.0, 1.0),
        ]
    }

    pub fn end(&self) -> &Point2d {
        &self.end
    }

    pub fn size(&self) -> &Size2d {
        &self.size
    }

    /// Returns the radius of the incircle.
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AABB;
    /// assert_eq!(AABB::simple(100, 100, 8, 6).inner_radius(), 3);
    /// assert_eq!(AABB::simple(100, 100, 6, 8).inner_radius(), 3);
    /// ```
    pub fn inner_radius(&self) -> u32 {
        self.size.width().min(self.size.height()) / 2
    }

    /// Is the [`Point2d`] inside?
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(10, 20, 30, 40);
    ///
    /// assert!(aabb.is_inside(&Point2d::new(25, 40)));
    /// assert!(!aabb.is_inside(&Point2d::new(0, 0)));
    /// ```
    pub fn is_inside(&self, point: &Point2d) -> bool {
        point.x >= self.start.x
            && point.y >= self.start.y
            && point.x < self.end.x
            && point.y < self.end.y
    }

    /// Gets a [`point`](Point2d) inside the axis aligned bounding box.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///   +---------------------> horizontal
    ///   |     0         1
    ///   |   0 *---------*
    ///   |     | *       |
    ///   |     |  point  |
    ///   |     |         |
    ///   |   1 *---------*
    ///   |
    ///   v
    /// vertical
    /// ```
    ///
    /// # Example
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(2, 3, 30, 60);
    ///
    /// assert_eq!(aabb.get_point(0.5, 0.25), Point2d::new(17, 18));
    /// ```
    pub fn get_point(&self, horizontal: f32, vertical: f32) -> Point2d {
        Point2d::new(
            self.start.x + (self.size.width() as f32 * horizontal) as i32,
            self.start.y + (self.size.height() as f32 * vertical) as i32,
        )
    }

    /// Shrinks the axis aligned bounding box by a certain amount?
    ///
    /// ```
    ///# use rpg_tools_core::model::math::aabb2d::AABB;
    ///# use rpg_tools_core::model::math::point2d::Point2d;
    /// let aabb = AABB::simple(1000, 2000, 400, 600);
    /// let desired = AABB::simple(1100, 2150, 200, 300);
    ///
    /// assert_eq!(aabb.scale(0.5), desired);
    /// ```
    pub fn scale(&self, scale: f32) -> Self {
        Self::with_center(self.center(), self.size.scale(scale, scale))
    }
}

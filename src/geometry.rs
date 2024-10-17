use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// ```
/// use crate::cp_rs::geometry::*;
/// let mut p = Point2D::new(1, 2);
/// let p2 = Point2D::new(3, 4);
/// p += p2;
/// assert!(p.x == 4);
/// assert!(p.y == 6);
/// ```
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Display for Point2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Debug for Point2D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T> Point2D<T> {
    /// Create a new 2d point with generic type T
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(1, 2);
    /// assert!(p.x == 1);
    /// assert!(p.y == 2);
    /// ```
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }
}

impl<T> Add<Point2D<T>> for Point2D<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    /// You can add two points together if T can be added.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(1, 2);
    /// let p2 = Point2D::new(1, 2);
    /// let p3 = p + p2;
    /// assert!(p3.x == 2);
    /// assert!(p3.y == 4);
    /// ```
    fn add(self, rhs: Point2D<T>) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign<Point2D<T>> for Point2D<T>
where
    T: std::ops::Add<Output = T> + Clone,
{
    /// You can add one point to another one, if T implements that as well.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let mut p = Point2D::new(1, 2);
    /// p += Point2D::new(1, 2);
    /// assert!(p.x == 2);
    /// assert!(p.y == 4);
    /// ```
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x.clone() + other.x,
            y: self.y.clone() + other.y,
        }
    }
}

impl<T> Sub<Point2D<T>> for Point2D<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;

    /// You can subtract two points, if T can be subtracted from each other.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(1, 2);
    /// let p2 = Point2D::new(2, 3);
    /// let p3 = p2 - p;
    /// assert!(p3.x == 1);
    /// assert!(p3.y == 1);
    /// ```
    fn sub(self, rhs: Point2D<T>) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign<Point2D<T>> for Point2D<T>
where
    T: std::ops::Sub<Output = T> + Clone,
{
    /// You can subtract a point from another point, if T implements that.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let mut p = Point2D::new(1, 2);
    /// p -= Point2D::new(1, 1);
    /// assert!(p.x == 0);
    /// assert!(p.y == 1);
    /// ```
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x.clone() - other.x,
            y: self.y.clone() - other.y,
        }
    }
}

impl<T, S> Mul<S> for Point2D<T>
where
    T: std::ops::Mul<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    /// You can multiply a point by a scalar S, if *: (T, S) -> T exists.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(1, 2);
    /// let p2 = p * 2;
    /// assert!(p2.x == 2);
    /// assert!(p2.y == 4);
    /// ```
    fn mul(self, rhs: S) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T, S> MulAssign<S> for Point2D<T>
where
    T: std::ops::Mul<S, Output = T> + Clone,
    S: Copy,
{
    /// You can scale a point by a scalar S, if *: (T, S) -> T exists.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let mut p = Point2D::new(2.0, 4.0);
    /// p *= 2.5;
    /// assert!(p.x == 5.0);
    /// assert!(p.y == 10.0);
    /// ```
    fn mul_assign(&mut self, other: S) {
        *self = Self {
            x: self.x.clone() * other,
            y: self.y.clone() * other,
        }
    }
}

impl<T, S> Div<S> for Point2D<T>
where
    T: std::ops::Div<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    /// You can a point by a scalar S, if /: (T, S) -> T exists.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(2, 4);
    /// let p2 = p / 2;
    /// assert!(p2.x == 1);
    /// assert!(p2.y == 2);
    /// ```
    fn div(self, rhs: S) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T, S> DivAssign<S> for Point2D<T>
where
    T: std::ops::Div<S, Output = T> + Clone,
    S: Copy,
{
    /// You can scale a point by a scalar S, if /: (T, S) -> T exists.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let mut p = Point2D::new(2, 4);
    /// p /= 2;
    /// assert!(p.x == 1);
    /// assert!(p.y == 2);
    /// ```
    fn div_assign(&mut self, other: S) {
        *self = Self {
            x: self.x.clone() / other,
            y: self.y.clone() / other,
        }
    }
}

impl<T, S> Rem<S> for Point2D<T>
where
    T: std::ops::Rem<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    /// You can get the remainder of a point, e.g. if you want to limit it to a square grid.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let mut p = Point2D::new(4, 5);
    /// p %= 3;
    /// assert!(p.x == 1);
    /// assert!(p.y == 2);
    /// ```
    fn rem(self, rhs: S) -> Self {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<T, S> RemAssign<S> for Point2D<T>
where
    T: std::ops::Rem<S, Output = T> + Clone,
    S: Copy,
{
    /// You can get the remainder of a point, e.g. if you want to limit it to a square grid.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(4, 5);
    /// let p2 = p % 3;
    /// assert!(p2.x == 1);
    /// assert!(p2.y == 2);
    /// ```
    fn rem_assign(&mut self, other: S) {
        *self = Self {
            x: self.x.clone() % other,
            y: self.y.clone() % other,
        }
    }
}

impl<T> Point2D<T>
where
    T: Into<f64> + Copy,
{
    /// Calculate the length of a point (Euclidean distance to (0, 0)).
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(3, 4);
    /// assert!(p.len() == 5.0);
    /// ```
    pub fn len(&self) -> f64 {
        let x: f64 = self.x.into();
        let y: f64 = self.y.into();
        (x * x + y * y).sqrt()
    }
    /// Compute the dot product of two points (find out the length of the projection of one point
    /// onto the other one.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let p = Point2D::new(1, 2);
    /// let p2 = Point2D::new(2, 1);
    /// assert!(p.dot(p2) == 4.0);
    /// ```
    pub fn dot(&self, other: Point2D<T>) -> f64 {
        self.x.into() * other.x.into() + self.y.into() * other.y.into()
    }
}

impl<T> Point2D<T>
where
    T: std::ops::Neg<Output = T> + Clone,
{
    /// Rotate clockwise by 90 degrees
    /// Create a new 2d point with generic type T
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let mut p = Point2D::new(1, 2);
    /// p.rotcw();
    /// assert!(p.x == 2);
    /// assert!(p.y == -1);
    /// ```
    pub fn rotcw(&mut self) {
        let y: T = self.y.clone();
        self.y = -self.x.clone();
        self.x = y;
    }
    /// Rotate counter-clockwise by 90 degrees
    /// Create a new 2d point with generic type T
    ///
    /// # Example
    ///
    /// ```
    /// use crate::cp_rs::geometry::*;
    ///
    /// let mut p = Point2D::new(1, 2);
    /// p.rotccw();
    /// assert!(p.x == -2);
    /// assert!(p.y == 1);
    /// ```
    pub fn rotccw(&mut self) {
        let x: T = self.x.clone();
        self.x = -self.y.clone();
        self.y = x;
    }
}

//! An implementation of a 2-dimensional point.
use std::{
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

// POINT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub const UP: Point = Point { x: 0, y: -1 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };

pub const ORTHO_DIRS: [Point; 4] = [UP, RIGHT, DOWN, LEFT];
pub const DIAGONALS: [Point; 8] = [
    UP,
    Point { x: 1, y: -1 },
    RIGHT,
    Point { x: 1, y: 1 },
    DOWN,
    Point { x: -1, y: 1 },
    LEFT,
    Point { x: -1, y: -1 },
];

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Point {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Point::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Point {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
    }
}

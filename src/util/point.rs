use std::{
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

pub const NORTH: Point = Point { x: 0, y: -1 };
pub const SOUTH: Point = Point { x: 0, y: 1 };
pub const EAST: Point = Point { x: 1, y: 0 };
pub const WEST: Point = Point { x: -1, y: 0 };
pub const SOUTH_EAST: Point = Point { x: 1, y: 1 };
pub const SOUTH_WEST: Point = Point { x: -1, y: 1 };

pub const DIRECTIONS: [Point; 8] = [
    Point::new(1, 0),
    Point::new(-1, 0),
    Point::new(0, 1),
    Point::new(0, -1),
    Point::new(1, 1),
    Point::new(-1, 1),
    Point::new(1, -1),
    Point::new(-1, -1),
];

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
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

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

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

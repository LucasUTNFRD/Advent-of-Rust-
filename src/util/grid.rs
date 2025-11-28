use std::{
    fmt,
    ops::{Index, IndexMut},
};

use crate::util::point::Point;

/// Flattened 1D array (Vec<T>)
/// let grid_1d = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
/// let width = 3;
///
/// // Access: grid_1d[width * y + x]
/// let value = grid_1d[width * 1 + 2]; // grid_1d[5] = 'F'
/// ```
///
/// **Memory Layout (identical for both)**
/// ```
/// Index:  0    1    2    3    4    5    6    7    8
/// Value: 'A' 'B' 'C' 'D' 'E' 'F' 'G' 'H' 'I'
/// Row 0: ───────────┘
/// Row 1:              ───────────┘
/// Row 2:                           ───────────┘
#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    height: i32,
    width: i32,
    data: Vec<T>,
}

// 1. Generic Debug: Prints raw values organized by row
// Usage: println!("{:?}", grid);
impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid ({}x{}):", self.width, self.height)?;
        for y in 0..self.height {
            write!(f, "  ")?; // Indent
            for x in 0..self.width {
                let val = &self.data[(self.width * y + x) as usize];
                write!(f, "{:?} ", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// 2. Specialized Display for u8: Prints the ASCII characters (Essential for AoC maps)
// Usage: println!("{}", grid);
impl fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Map View ({}x{}):", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.data[(self.width * y + x) as usize];
                write!(f, "{}", val as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();

        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let bytes = raw.concat();

        Grid {
            width,
            height,
            data: bytes,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn find(&self, element: T) -> Option<Point> {
        self.data.iter().position(|&e| e == element).map(|idx| {
            let x = (idx as i32) % self.width;
            let y = (idx as i32) / self.width;
            Point::new(x, y)
        })
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if point.x >= self.width && point.y >= self.height {
            return None;
        }
        Some(&self[point])
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.data[(self.width * index.0 + index.1) as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        &mut self.data[(self.width * index.0 + index.1) as usize]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[(self.width * index.y + index.x) as usize]
    }
}

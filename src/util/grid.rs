use std::{
    fmt::{self, Debug},
    ops::{Index, IndexMut},
};

use crate::util::point::Point;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub height: i32,
    pub width: i32,
    pub data: Vec<T>,
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

    pub fn copy_maze_with(&self, symbol: u8) -> Self {
        let mut s = self.clone();
        s.data.iter_mut().for_each(|b| *b = symbol);
        s
    }
}

impl<T: Copy + PartialEq + Debug> Grid<T> {
    pub fn find(&self, element: T) -> Option<Point> {
        self.data.iter().position(|&e| e == element).map(|idx| {
            let x = (idx as i32) % self.width;
            let y = (idx as i32) / self.width;
            Point::new(x, y)
        })
    }

    pub fn find_all(&self, element: T) -> Vec<Point> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, e)| {
                if *e == element {
                    // Calculate point for a match
                    let x = (idx as i32) % self.width;
                    let y = (idx as i32) / self.width;
                    Some(Point::new(x, y)) // Return Some(Point)
                } else {
                    None // Return None to discard non-matches
                }
            })
            .collect() // Collects into Vec<Point>
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if point.x >= self.width || point.y >= self.height || point.x < 0 || point.y < 0 {
            return None;
        }
        Some(&self[point])
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut T> {
        if point.x >= self.width || point.y >= self.height || point.x < 0 || point.y < 0 {
            return None;
        }
        Some(&mut self[point])
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.data[(self.width * index.1 + index.0) as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        &mut self.data[(self.width * index.1 + index.0) as usize]
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

impl<T: Copy> Grid<T> {
    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid {
            width,
            height,
            data: vec![value; (width * height) as usize],
        }
    }
}

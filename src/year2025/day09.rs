use std::collections::VecDeque;

use crate::util::{
    grid::Grid,
    point::{DIRECTIONS, NORTH, Point, WEST},
};
use itertools::Itertools;
use rayon::slice::ParallelSliceMut;
use rustc_hash::FxHashMap as HashMap;

pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part_1(coords: &[Point]) -> u64 {
    coords
        .iter()
        .tuple_combinations()
        .map(|(x, y)| area(x, y))
        .max()
        .unwrap()
}

pub fn part_2(coords: &[Point]) -> u64 {
    let mut x_vals: Vec<i32> = coords.iter().map(|p| p.x).collect();
    let mut y_vals: Vec<i32> = coords.iter().map(|p| p.y).collect();

    x_vals.push(i32::MIN);
    x_vals.push(i32::MAX);
    y_vals.push(i32::MIN);
    y_vals.push(i32::MAX);

    x_vals.sort_unstable();
    x_vals.dedup();
    y_vals.sort_unstable();
    y_vals.dedup();

    let x_map: HashMap<i32, i32> = x_vals
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i as i32))
        .collect();
    let y_map: HashMap<i32, i32> = y_vals
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i as i32))
        .collect();

    let shrunk: Vec<Point> = coords
        .iter()
        .map(|p| Point::new(x_map[&p.x], y_map[&p.y]))
        .collect();

    let mut grid = Grid::new(x_vals.len() as i32, y_vals.len() as i32, 2i64);

    for i in 0..coords.len() {
        let p1 = shrunk[i];
        let p2 = shrunk[(i + 1) % coords.len()];
        let (min_x, min_y, max_x, max_y) = (
            p1.x.min(p2.x),
            p1.y.min(p2.y),
            p1.x.max(p2.x),
            p1.y.max(p2.y),
        );

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                grid[Point::new(x, y)] = 1; // INSIDE
            }
        }
    }

    // Flood fill from origin (guaranteed exterior due to sentinels)
    let mut queue = VecDeque::new();
    queue.push_back(Point::new(0, 0));
    grid[Point::new(0, 0)] = 0; // OUTSIDE

    while let Some(point) = queue.pop_front() {
        for dir in &DIRECTIONS[0..4] {
            let next = point + *dir;
            if let Some(val) = grid.get_mut(next)
                && *val == 2
            {
                *val = 0;
                queue.push_back(next);
            }
        }
    }

    // Build prefix sum: grid[x,y] = count of valid cells in rectangle from (0,0) to (x,y)
    for y in 1..grid.height {
        for x in 1..grid.width {
            let p = Point::new(x, y);
            let valid = i64::from(grid[p] != 0);
            grid[p] = valid + grid[p + NORTH] + grid[p + WEST] - grid[p + NORTH + WEST];
        }
    }

    let mut max_area = 0u64;

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let (x1, y1, x2, y2) = (
                shrunk[i].x.min(shrunk[j].x),
                shrunk[i].y.min(shrunk[j].y),
                shrunk[i].x.max(shrunk[j].x),
                shrunk[i].y.max(shrunk[j].y),
            );

            let expected = ((x2 - x1 + 1) * (y2 - y1 + 1)) as i64;
            let actual = grid[Point::new(x2, y2)]
                - grid[Point::new(x1 - 1, y2)]
                - grid[Point::new(x2, y1 - 1)]
                + grid[Point::new(x1 - 1, y1 - 1)];

            if expected == actual {
                let area = area(&coords[i], &coords[j]);
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

#[repr(i64)]
enum CellType {
    Unknown = 0i64,
    Boundary = 1i64,
    Interior = 2i64,
    Exterior = 3i64,
}

#[inline]
pub fn area(p1: &Point, p2: &Point) -> u64 {
    let x_distance = p1.x.abs_diff(p2.x) as u64;
    let y_distance = p1.y.abs_diff(p2.y) as u64;

    (x_distance + 1) * (y_distance + 1)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_area() {
        assert_eq!(50, area(&Point { x: 2, y: 5 }, &Point { x: 11, y: 1 }))
    }

    const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_2() {
        assert_eq!(24, part_2(&parse(SAMPLE_INPUT)))
    }
}

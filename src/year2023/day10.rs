use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;

use crate::util::{
    grid::Grid,
    point::{EAST, NORTH, SOUTH, WEST},
};

pub fn part_1(input: &str) -> u64 {
    let grid = Grid::parse(input);
    let starting_position = grid.find(b'S').unwrap();

    let mut q = VecDeque::new();
    q.push_back(starting_position);

    let mut visited = HashSet::default();
    visited.insert(starting_position);

    while let Some(curr_node) = q.pop_front() {
        let curr_char = grid[curr_node];

        let directions = match curr_char {
            b'|' => vec![NORTH, SOUTH],
            b'-' => vec![EAST, WEST],
            b'L' => vec![NORTH, EAST],
            b'J' => vec![NORTH, WEST],
            b'7' => vec![SOUTH, WEST],
            b'F' => vec![SOUTH, EAST],
            b'S' => vec![NORTH, SOUTH, EAST, WEST],
            _ => continue,
        };

        for d in directions {
            let neighbor_point = curr_node + d;

            if let Some(&neighbor_char) = grid.get(neighbor_point) {
                let is_valid_connection = matches!(
                    (d, neighbor_char),
                    (NORTH, b'|' | b'7' | b'F' | b'S')
                        | (SOUTH, b'|' | b'L' | b'J' | b'S')
                        | (EAST, b'-' | b'J' | b'7' | b'S')
                        | (WEST, b'-' | b'L' | b'F' | b'S')
                );

                if is_valid_connection && !visited.contains(&neighbor_point) {
                    visited.insert(neighbor_point);
                    q.push_back(neighbor_point);
                }
            }
        }
    }

    visited.len() as u64 / 2
}

pub fn part_2(input: &str) -> u64 {
    let grid = Grid::parse(input);

    let starting_position = grid.find(b'S').unwrap();

    let mut q = VecDeque::new();
    q.push_back(starting_position);

    let mut visited = HashSet::default();
    visited.insert(starting_position);

    while let Some(curr_node) = q.pop_front() {
        let curr_char = grid[curr_node];

        let directions = match curr_char {
            b'|' => vec![NORTH, SOUTH],
            b'-' => vec![EAST, WEST],
            b'L' => vec![NORTH, EAST],
            b'J' => vec![NORTH, WEST],
            b'7' => vec![SOUTH, WEST],
            b'F' => vec![SOUTH, EAST],
            b'S' => vec![NORTH, SOUTH, EAST, WEST],
            _ => continue,
        };

        for d in directions {
            let neighbor_point = curr_node + d;

            if let Some(&neighbor_char) = grid.get(neighbor_point) {
                let is_valid_connection = matches!(
                    (d, neighbor_char),
                    (NORTH, b'|' | b'7' | b'F' | b'S')
                        | (SOUTH, b'|' | b'L' | b'J' | b'S')
                        | (EAST, b'-' | b'J' | b'7' | b'S')
                        | (WEST, b'-' | b'L' | b'F' | b'S')
                );

                if is_valid_connection && !visited.contains(&neighbor_point) {
                    visited.insert(neighbor_point);
                    q.push_back(neighbor_point);
                }
            }
        }
    }

    todo!()
}

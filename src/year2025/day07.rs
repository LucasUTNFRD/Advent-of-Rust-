use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

use crate::util::{
    grid::Grid,
    point::{Point, SOUTH, SOUTH_EAST, SOUTH_WEST},
};

pub fn parse(input: &str) -> (Grid<u8>, Point) {
    let grid = Grid::parse(input);
    let starting_point = grid.find(b'S').unwrap();
    (grid, starting_point)
}

pub fn part_1((grid, starting_point): &(Grid<u8>, Point)) -> u64 {
    let mut visited: HashSet<Point> = HashSet::default();
    let mut stack = vec![*starting_point];
    visited.insert(*starting_point);

    let mut count = 0;

    while let Some(p) = stack.pop() {
        let mut next_step = p + SOUTH;
        loop {
            if !visited.insert(next_step) {
                break;
            }

            match grid.get(next_step) {
                Some(b) => {
                    if *b == b'.' {
                        next_step += SOUTH;
                        continue; //continue downward
                    } else if *b == b'^' {
                        // we found a beam split
                        count += 1;
                        stack.push(next_step + SOUTH_EAST);
                        stack.push(next_step + SOUTH_WEST);
                        break; // break loop and go to next path
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    count
}

pub fn part_2((grid, starting_point): &(Grid<u8>, Point)) -> u64 {
    fn solve(p: Point, grid: &Grid<u8>, memo: &mut HashMap<Point, u64>) -> u64 {
        if let Some(&cached) = memo.get(&p) {
            return cached;
        }

        match grid.get(p) {
            Some(b'.') | Some(b'S') => {
                let result = solve(p + SOUTH, grid, memo);
                memo.insert(p, result);
                result
            }
            Some(b'^') => {
                let left = solve(p + SOUTH_WEST, grid, memo);
                let right = solve(p + SOUTH_EAST, grid, memo);
                let result = left + right;
                memo.insert(p, result);
                result
            }
            None => 1,
            _ => 0,
        }
    }

    let mut memo = HashMap::default();
    solve(*starting_point, grid, &mut memo)
}

#[cfg(test)]
mod test {
    const SAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    use super::*;
    #[test]
    fn test_part_1() {
        let input = parse(SAMPLE_INPUT);
        assert_eq!(21, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = parse(SAMPLE_INPUT);
        assert_eq!(40, part_2(&input));
    }
}

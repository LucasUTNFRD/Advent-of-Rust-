use crate::util::{
    grid::Grid,
    point::{DIRECTIONS, Point},
};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part_1(grid: &Grid<u8>) -> u32 {
    find_all_paper_rolls(grid)
        .filter(|&point| {
            let mut count = 0;
            for d in DIRECTIONS {
                let new_dir = point + d;
                count += grid.get(new_dir).map_or(0, |e| u32::from(*e == b'@'))
            }
            count < 4
        })
        .count() as u32
}

const PAPER_ROLL: u8 = b'@';

fn find_all_paper_rolls(grid: &Grid<u8>) -> impl Iterator<Item = Point> {
    grid.data.iter().enumerate().filter_map(|(idx, e)| {
        if *e == PAPER_ROLL {
            // Calculate point for a match
            let x = (idx as i32) % grid.width;
            let y = (idx as i32) / grid.width;
            Some(Point::new(x, y)) // Return Some(Point)
        } else {
            None // Return None to discard non-matches
        }
    })
}

pub fn part_2(grid: Grid<u8>) -> u64 {
    // let paper_rolls_points =
    let mut grid = grid;
    let mut total_removed = 0;
    let mut curr_removed = 1;

    while curr_removed != 0 {
        curr_removed = 0;

        let points_to_remove = find_all_paper_rolls(&grid)
            .filter(|&point| {
                if grid[point] != b'@' {
                    return false;
                }
                let count = DIRECTIONS
                    .iter()
                    .filter(|&&d| grid.get(point + d).is_some_and(|&e| e == b'@'))
                    .count();
                count < 4
            })
            .collect::<Vec<_>>();

        for point in points_to_remove {
            grid[point] = b'X';
            curr_removed += 1;
        }

        total_removed += curr_removed;
    }
    total_removed
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(13, part_1(&input))
    }

    #[test]
    fn test_part_2() {
        let input = parse(TEST_INPUT);
        assert_eq!(43, part_2(input))
    }
}

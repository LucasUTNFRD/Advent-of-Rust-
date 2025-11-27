use std::collections::HashSet;

use crate::util::point::DIRECTIONS;

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

// optimization strat
// for imporving cache locality we can represent a 2d array into a 1d array where instead of
// indexing as [x][y] we do [width*x+y]

pub fn part_1(input: &[&[u8]]) -> usize {
    let n = input.len();
    let m = input[0].len();

    let mut nums = Vec::new();
    let mut seen = HashSet::new();

    for y in 0..n {
        for x in 0..m {
            if input[y][x] != b'.'
                && input[y][x].is_ascii_graphic()
                && !input[y][x].is_ascii_digit()
            {
                for p in &DIRECTIONS {
                    let (dx, dy) = p.coords();
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 && (ny as usize) < n && (nx as usize) < m {
                        let (nx, ny) = (nx as usize, ny as usize);

                        if input[ny][nx].is_ascii_digit() {
                            let (num, start_x) = get_whole_number(input, nx, ny);

                            if seen.insert((ny, start_x)) {
                                nums.push(num);
                            }
                        }
                    }
                }
            }
        }
    }

    nums.iter().sum()
}

fn get_whole_number(input: &[&[u8]], x: usize, y: usize) -> (usize, usize) {
    let mut start = x;
    while start > 0 && input[y][start - 1].is_ascii_digit() {
        start -= 1;
    }

    let mut end = x;
    while end < input[y].len() && input[y][end].is_ascii_digit() {
        end += 1;
    }

    let num_str = unsafe { std::str::from_utf8_unchecked(&input[y][start..end]) };
    let num = num_str.parse::<usize>().unwrap();

    (num, start)
}

pub fn part_2(input: &[&[u8]]) -> usize {
    let n = input.len();
    let m = input[0].len();

    let mut nums = Vec::new();
    let mut seen = HashSet::new();

    for y in 0..n {
        for x in 0..m {
            if input[y][x] == b'*' {
                let mut adjacence = Vec::new();

                for p in &DIRECTIONS {
                    let (dx, dy) = p.coords();
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 && (ny as usize) < n && (nx as usize) < m {
                        let (nx, ny) = (nx as usize, ny as usize);

                        if input[ny][nx].is_ascii_digit() {
                            let (num, start_x) = get_whole_number(input, nx, ny);

                            if seen.insert((ny, start_x)) {
                                // nums.push(num);
                                adjacence.push(num);
                            }
                        }
                    }
                }

                if adjacence.len() == 2 {
                    nums.push(adjacence[0] * adjacence[1]);
                }
            }
        }
    }

    nums.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::year2023::day3::{parse, part_1, part_2};

    const SAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const SAMPLE_INPUT_2: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(4361, part_1(&parse(SAMPLE_INPUT)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4361, part_2(&parse(SAMPLE_INPUT_2)));
    }
}

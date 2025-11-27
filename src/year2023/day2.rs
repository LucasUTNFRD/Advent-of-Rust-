pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

const CONFIG: (u32, u32, u32) = (12, 13, 14); // r g b

pub fn part_1(input: &[&str]) -> usize {
    input
        .iter()
        .enumerate()
        .map(
            |(game_id, game_line)| {
                if possible(game_line) { game_id + 1 } else { 0 }
            },
        )
        .sum()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cube(u32, u32, u32); // r g b

impl From<&str> for Cube {
    fn from(line: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pair in line.split(", ") {
            let mut parts = pair.split_whitespace();

            let count_str = parts.next().unwrap();
            let count = count_str.parse::<u32>().unwrap();

            let color = parts.next().unwrap();

            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => (),
            }
        }
        Self(red, green, blue)
    }
}

fn possible(game: &str) -> bool {
    let (_, game_data) = game.split_once(": ").unwrap();
    game_data.split(';').map(|s| s.trim()).all(|x| {
        let cube = Cube::from(x);
        cube.0 <= CONFIG.0 && cube.1 <= CONFIG.1 && cube.2 <= CONFIG.2
    })
}

pub fn part_2(input: &[&str]) -> u32 {
    // input.iter().enumerate().map(todo!()).sum()
    input
        .iter()
        .map(|line| {
            let (_, game_data) = line.split_once(": ").unwrap();
            let max_cube = game_data.split(';').map(|s| s.trim()).map(Cube::from).fold(
                Cube(0, 0, 0),
                |acc, cube| {
                    Cube(
                        acc.0.max(cube.0), // Max red
                        acc.1.max(cube.1), // Max green
                        acc.2.max(cube.2), // Max blue
                    )
                },
            );

            max_cube.0 * max_cube.1 * max_cube.2
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::year2023::day2::{Cube, parse, part_1};

    const SAMPLE_INPUT_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        let result = part_1(&parse(SAMPLE_INPUT_1));
        assert_eq!(result, 8)
    }

    #[test]
    fn test_cube_from_str() {
        let draw_string_1 = "3 blue, 4 red";
        let cube = Cube::from(draw_string_1);
        assert_eq!(cube, Cube(4, 0, 3));
    }
}

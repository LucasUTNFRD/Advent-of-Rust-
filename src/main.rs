use std::{fs::read_to_string, time::Instant};

use aoc_rs::year2025;

fn main() {
    let data = read_to_string("inputs/year2025/day04.txt").unwrap();

    let input = year2025::day04::parse(&data);

    let start = Instant::now();
    let solution_part_1 = year2025::day04::part_1(&input);
    let duration_part_1 = start.elapsed();
    println!(
        "Day 4 - Part 1 solution {} - elapsed {:#?}",
        solution_part_1, duration_part_1
    );

    let start = Instant::now();
    let solution_part_2 = year2025::day04::part_2(input);
    let duration_part_2 = start.elapsed();
    println!(
        "Day 4 - Part 2 solution {} - elapsed {:#?}",
        solution_part_2, duration_part_2
    );
}

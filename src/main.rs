use std::{fs::read_to_string, time::Instant};

use aoc_rs::year2023;

fn main() {
    let data = read_to_string("inputs/year2023/day10.txt").unwrap();
    // let input = year2023::day10::parse(&data);

    let start = Instant::now();
    let solution_part_1 = year2023::day10::part_1(&data);
    let duration_part_1 = start.elapsed();
    println!(
        "Day 10 - Part 1 solution {} - elapsed {:#?}",
        solution_part_1, duration_part_1
    );
    //
    // let start = Instant::now();
    // let solution_part_1 = year2023::day10::part_2(&input);
    // let duration_part_1 = start.elapsed();
    // println!(
    //     "Day 2 - Part 1 solution {} - elapsed {:#?}",
    //     solution_part_1, duration_part_1
    // );
}

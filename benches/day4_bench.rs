use criterion::{Criterion, criterion_group, criterion_main};
use rustc_hash::FxHashSet;
use std::{fs::read_to_string, hint::black_box};

// Static bool array approach (current implementation)
type Set = [bool; 100];

fn parse_static_array(input: &str) -> Vec<(Set, Set)> {
    input
        .lines()
        .map(|line| {
            let (winning_str, having_str) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let mut winning_mask = [false; 100];

            for n in winning_str
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
            {
                winning_mask[n] = true;
            }

            let mut having_mask = [false; 100];
            for n in having_str
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
            {
                having_mask[n] = true;
            }

            (winning_mask, having_mask)
        })
        .collect()
}

fn part_1_static_array(input: &[(Set, Set)]) -> u32 {
    input
        .iter()
        .map(|(winning, having)| {
            let mut match_count = 0;
            for k in 0..100 {
                if winning[k] && having[k] {
                    match_count += 1;
                }
            }
            if match_count == 0 {
                0
            } else {
                1 << (match_count - 1)
            }
        })
        .sum()
}

fn part_2_static_array(input: &[(Set, Set)]) -> u32 {
    let mut counts = vec![1; input.len()];
    for i in 0..input.len() {
        let (winning, having) = &input[i];
        let current_instances = counts[i];

        let mut match_count = 0;
        for k in 0..100 {
            if winning[k] && having[k] {
                match_count += 1;
            }
        }

        if match_count > 0 {
            let end_index = (i + match_count + 1).min(input.len());
            for counts in counts.iter_mut().take(end_index).skip(i + 1) {
                *counts += current_instances;
            }
        }
    }
    counts.iter().sum()
}

// FxHashSet approach
fn parse_hashset(input: &str) -> Vec<(FxHashSet<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (winning_str, having_str) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();

            let winning: FxHashSet<u32> = winning_str
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            let having: Vec<u32> = having_str
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            (winning, having)
        })
        .collect()
}

fn part_1_hashset(input: &[(FxHashSet<u32>, Vec<u32>)]) -> u32 {
    input
        .iter()
        .map(|(winning, having)| {
            let match_count = having.iter().filter(|n| winning.contains(n)).count();
            if match_count == 0 {
                0
            } else {
                1 << (match_count - 1)
            }
        })
        .sum()
}

fn part_2_hashset(input: &[(FxHashSet<u32>, Vec<u32>)]) -> u32 {
    let mut counts = vec![1; input.len()];
    for i in 0..input.len() {
        let (winning, having) = &input[i];
        let current_instances = counts[i];

        let match_count = having.iter().filter(|n| winning.contains(n)).count();

        if match_count > 0 {
            let end_index = (i + match_count + 1).min(input.len());
            for counts in counts.iter_mut().take(end_index).skip(i + 1) {
                *counts += current_instances;
            }
        }
    }
    counts.iter().sum()
}

fn benchmark_day4(c: &mut Criterion) {
    let input_data = read_to_string("inputs/year2023/day4.txt").expect("Failed to read input file");

    let mut group = c.benchmark_group("day4");

    // Parse benchmarks
    group.bench_function("parse_static_array", |b| {
        b.iter(|| parse_static_array(black_box(&input_data)))
    });

    group.bench_function("parse_hashset", |b| {
        b.iter(|| parse_hashset(black_box(&input_data)))
    });

    // Part 1 benchmarks
    let parsed_static = parse_static_array(&input_data);
    let parsed_hashset = parse_hashset(&input_data);

    group.bench_function("part1_static_array", |b| {
        b.iter(|| part_1_static_array(black_box(&parsed_static)))
    });

    group.bench_function("part1_hashset", |b| {
        b.iter(|| part_1_hashset(black_box(&parsed_hashset)))
    });

    // Part 2 benchmarks
    group.bench_function("part2_static_array", |b| {
        b.iter(|| part_2_static_array(black_box(&parsed_static)))
    });

    group.bench_function("part2_hashset", |b| {
        b.iter(|| part_2_hashset(black_box(&parsed_hashset)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_day4);
criterion_main!(benches);

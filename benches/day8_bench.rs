use criterion::{Criterion, criterion_group, criterion_main};
use rayon::prelude::*;
use std::{collections::HashMap, fs::read_to_string, hint::black_box};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

// Original parallel version
fn part_2_parallel(input: &str) -> usize {
    let (instructions, elements_str) = input.split_once("\n\n").unwrap();
    let mut elements: HashMap<&str, (&str, &str)> = HashMap::default();
    let mut starting_nodes = vec![];

    for l in elements_str.lines() {
        let key = &l[0..3];
        if key.ends_with('A') {
            starting_nodes.push(key);
        }
        elements.insert(key, (&l[7..10], &l[12..15]));
    }

    starting_nodes
        .par_iter()
        .map(|&start| {
            let mut node = start;
            let mut steps = 0;
            for i in instructions.chars().cycle() {
                if node.ends_with('Z') {
                    break;
                }
                steps += 1;
                node = if i == 'L' {
                    elements[node].0
                } else {
                    elements[node].1
                }
            }
            steps
        })
        .reduce(|| 1, lcm)
}

// Sequential version
fn part_2_sequential(input: &str) -> usize {
    let (instructions, elements_str) = input.split_once("\n\n").unwrap();
    let mut elements: HashMap<&str, (&str, &str)> = HashMap::default();
    let mut starting_nodes = vec![];

    for l in elements_str.lines() {
        let key = &l[0..3];
        if key.ends_with('A') {
            starting_nodes.push(key);
        }
        elements.insert(key, (&l[7..10], &l[12..15]));
    }

    starting_nodes
        .iter()
        .map(|&start| {
            let mut node = start;
            let mut steps = 0;
            for i in instructions.chars().cycle() {
                if node.ends_with('Z') {
                    break;
                }
                steps += 1;
                node = if i == 'L' {
                    elements[node].0
                } else {
                    elements[node].1
                }
            }
            steps
        })
        .fold(1, lcm)
}

fn benchmark_day8(c: &mut Criterion) {
    let input_data = read_to_string("inputs/year2023/day8.txt").expect("Failed to read input file");

    let mut group = c.benchmark_group("day8_part2");

    group.bench_function("parallel", |b| {
        b.iter(|| part_2_parallel(black_box(&input_data)))
    });

    group.bench_function("sequential", |b| {
        b.iter(|| part_2_sequential(black_box(&input_data)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_day8);
criterion_main!(benches);

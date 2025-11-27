use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap as HashMap;

const GOAL: &str = "ZZZ";

pub fn part_2(input: &str) -> usize {
    let (instructions, elements_str) = input.split_once("\n\n").unwrap();
    let mut elements: HashMap<&str, (&str, &str)> = HashMap::default();
    // nodes that ends with A
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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_1(input: &str) -> usize {
    let (instructions, elements_str) = input.split_once("\n\n").unwrap();
    let mut elements: HashMap<&str, (&str, &str)> = HashMap::default();
    for l in elements_str.lines() {
        let key = &l[0..3];
        let val = (&l[7..10], &l[12..15]);
        elements.insert(key, val);
    }

    let mut next_node = "AAA";
    let mut steps = 0;

    for i in instructions.chars().cycle() {
        if next_node == GOAL {
            break;
        }
        steps += 1;
        let (l, r) = elements[next_node];
        next_node = if i == 'L' { l } else { r }
    }

    steps
}

use std::{collections::VecDeque, ops::BitXor};

use bitvec::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Indicator {
    bytes: u16,
}

impl BitXor for Indicator {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bytes: self.bytes.bitxor(rhs.bytes),
        }
    }
}

impl Indicator {
    pub fn parse(input: &str) -> Indicator {
        let mut bits = 0u16;
        for (i, c) in input.chars().enumerate() {
            if c == '#' {
                bits |= 1 << i;
            }
        }
        Indicator { bytes: bits }
    }

    pub fn parse_bit_set(input: &str) -> Indicator {
        let mut bits: u16 = 0;
        for index_str in input.split(',') {
            if let Ok(index) = index_str.trim().parse::<u16>() {
                bits |= 1 << index;
            }
        }
        Indicator { bytes: bits }
    }
}

// The manual describes one machine per line. Each line contains a single indicator light diagram in [square brackets], one or more button wiring schematics in (parentheses), and joltage requirements in {curly braces}.
fn line_parser(line: &str) -> (Indicator, Vec<Indicator>, Vec<u32>) {
    let bracket_start = line.find("[").unwrap() + 1;
    let bracket_end = line.find("]").unwrap();

    let indicator = Indicator::parse(&line[bracket_start..bracket_end]);

    let curly_start = line.find('{').unwrap();
    let curly_end = line.find('}').unwrap();

    let joltage = line[curly_start + 1..curly_end]
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let button_section = &line[bracket_end + 1..curly_start];
    let button_wiring = button_section
        .split(')')
        .filter(|s| s.contains('('))
        .map(|s| {
            let content = s.trim().trim_start_matches('(');
            Indicator::parse_bit_set(content)
        })
        .collect();

    (indicator, button_wiring, joltage)
}

pub fn part_1(input: &str) -> u32 {
    let parsed_lines = input.lines().map(line_parser);

    parsed_lines
        .map(|(indicator_light, buttom_wiring, _)| {
            min_buttom_press(indicator_light, buttom_wiring.as_slice()) as u32
        })
        .sum()
}

fn min_buttom_press(expected: Indicator, wirings: &[Indicator]) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = bitvec![u8, Lsb0; 0; 65536];

    queue.push_back((Indicator { bytes: 0 }, 0));
    visited.set(0, true);

    while let Some((curr, depth)) = queue.pop_front() {
        for &wiring in wirings {
            let next = curr ^ wiring;

            if next == expected {
                return depth + 1;
            }

            if !visited[next.bytes as usize] {
                visited.set(next.bytes as usize, true);
                queue.push_back((next, depth + 1));
            }
        }
    }

    unreachable!()
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

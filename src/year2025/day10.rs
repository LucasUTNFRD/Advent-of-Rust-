use std::{collections::VecDeque, ops::BitXor};

use bitvec::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use z3::ast::Int;
use z3::{Optimize, SatResult};

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

    pub fn is_set(&self, index: u8) -> bool {
        // Matches the logic in parse: bits |= 1 << i
        (self.bytes & (1 << index)) != 0
    }
}

// The manual describes one machine per line. Each line contains a single indicator light diagram in [square brackets], one or more button wiring schematics in (parentheses), and joltage requirements in {curly braces}.
fn line_parser(line: &str) -> Input {
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

    Input {
        indicator,
        button_wiring,
        joltage,
    }
}

#[derive(Debug)]
pub struct Input {
    indicator: Indicator,
    button_wiring: Vec<Indicator>,
    pub(crate) joltage: Vec<u32>,
}

pub fn parse(input: &str) -> Vec<Input> {
    input.lines().map(line_parser).collect()
}

pub fn part_1(input: &[Input]) -> u32 {
    input
        .iter()
        .map(|input| min_buttom_press(input.indicator, input.button_wiring.as_slice()) as u32)
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

    0
}

pub fn part_2(input: &[Input]) -> u32 {
    input
        .par_iter()
        .map(|input| {
            min_buttom_press_constrained_by_joltage(
                input.button_wiring.as_slice(),
                input.joltage.as_slice(),
            )
        })
        .sum()
}

fn min_buttom_press_constrained_by_joltage(wirings: &[Indicator], joltage: &[u32]) -> u32 {
    let opt = Optimize::new();

    let x: Vec<Int> = (0..wirings.len())
        .map(|i| Int::new_const(format!("btn_{}", i)))
        .collect();

    let zero = Int::from_i64(0);

    for var in &x {
        opt.assert(&var.ge(&zero));
    }

    // 4. Build Linear Equations
    // For each joltage counter (row in the system), sum the contributions of relevant buttons.
    for (counter_idx, &target_val) in joltage.iter().enumerate() {
        let mut sum_expr = Int::from_i64(0);

        for (btn_idx, wiring) in wirings.iter().enumerate() {
            // Your is_set helper is perfect here
            if wiring.is_set(counter_idx as u8) {
                sum_expr = &sum_expr + &x[btn_idx];
            }
        }

        // Constraint: Sum of button presses == Target Joltage
        let target = Int::from_i64(target_val as i64);
        opt.assert(&sum_expr.eq(&target));
    }

    let total_presses = x.iter().fold(Int::from_i64(0), |acc, v| acc + v);
    opt.minimize(&total_presses);

    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            model
                .eval(&total_presses, true)
                .and_then(|r| r.as_u64())
                .unwrap_or(0) as u32
        }
        _ => unreachable!(),
    }
}

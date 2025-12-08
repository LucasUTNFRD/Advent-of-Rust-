use std::{ops::Range, vec};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Ops {
    Add,
    Mul,
}

pub struct Input {
    ops: Vec<Ops>,
    inputs: Vec<Vec<u64>>,
}

pub fn parse(input: &str) -> Input {
    let mut lines = input.lines().rev();

    let ops: Vec<Ops> = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| if c == '*' { Ops::Mul } else { Ops::Add })
        .collect();

    let inputs: Vec<Vec<u64>> = lines
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    Input { ops, inputs }
}
pub fn part_1(input: &Input) -> u64 {
    let mut outputs: Vec<u64> = input
        .ops
        .iter()
        .map(|&op| if op == Ops::Add { 0 } else { 1 })
        .collect();

    for (idx, &op) in input.ops.iter().enumerate() {
        let op: fn(u64, u64) -> u64 = match op {
            Ops::Add => |a, b| a + b,
            Ops::Mul => |a, b| a * b,
        };

        for input_vec in input.inputs.iter() {
            outputs[idx] = op(outputs[idx], input_vec[idx]);
        }
    }

    outputs.iter().sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut lines = input.lines().rev();

    let mut ops = vec![];
    let mut ops_idx = vec![];

    for (idx, op) in lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_idx, char)| !char.is_whitespace())
    {
        if op == '*' {
            ops.push(Ops::Mul);
        } else {
            ops.push(Ops::Add);
        }

        ops_idx.push(idx);
    }

    let lines: Vec<_> = lines.collect();
    let line_len = lines[0].len();

    let mut range_vec: Vec<Range<usize>> = Vec::new();
    for window in ops_idx.windows(2) {
        let start = window[0];
        let end = window[1] - 1; // The range ends just before the next start
        range_vec.push(start..end);
    }

    if let Some(&last_start) = ops_idx.last() {
        range_vec.push(last_start..line_len);
    }

    let mut problems = Vec::new();

    for range in range_vec.iter() {
        let mut problem = Vec::new();
        for line in lines.iter() {
            let sub_problem = line[range.clone()].to_string();
            problem.push(sub_problem);
        }
        problems.push(problem);
    }

    let mut count = 0;
    for (cols, &ops) in problems.iter().zip(ops.iter()) {
        let str_len = cols[0].len();
        let col_len = cols.len();

        // str_len vec of strs of col_len len
        let mut problems = vec![String::with_capacity(col_len); str_len];
        for sub_cols in cols.iter().rev() {
            for (idx, c) in sub_cols.chars().enumerate() {
                problems[idx].push(c);
            }
        }

        let problems = problems.iter().map(|s| s.trim().parse::<u64>().unwrap());
        count += match ops {
            Ops::Add => problems.sum::<u64>(),
            Ops::Mul => problems.product(),
        };
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    #[test]
    fn test_part_1() {
        let input = parse(SAMPLE_INPUT);
        assert_eq!(4277556, part_1(&input))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3263827, part_2(SAMPLE_INPUT))
    }
}

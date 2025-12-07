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
    // let amount_of_outputs = input.ops.len();
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

// GOD SAVE US ALL
pub fn part_2(input: &Input) -> u64 {
    todo!()
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
}

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

pub fn part_1(input: &[&[u8]]) -> u64 {
    input
        .iter()
        .map(|&bank| {
            let mut max_joltage: u64 = 0;
            let len = bank.len();
            for i in 0..len {
                let base = (bank[i] - b'0') * 10;
                for b in bank.iter().take(len).skip(i + 1) {
                    let curr_joltage = base + b - b'0';
                    max_joltage = max_joltage.max(u64::from(curr_joltage))
                }
            }
            max_joltage
        })
        .sum()
}
pub fn part_2(input: &[&[u8]]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(357, part_1(&input))
    }
}

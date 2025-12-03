pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

pub fn part_1(input: &[&[u8]]) -> u64 {
    input
        .iter()
        .map(|&bank| {
            // // O(N^2 ) solution``
            // Day 3 - Part 1 solution 17535 - elapsed 608.222µs
            // let mut max_joltage: u64 = 0;
            // let len = bank.len();
            // for i in 0..len {
            //     let base = (bank[i] - b'0') * 10;
            //     for b in bank.iter().take(len).skip(i + 1) {
            //         let curr_joltage = base + b - b'0';
            //         max_joltage = max_joltage.max(u64::from(curr_joltage))
            //     }
            // }
            //
            // max_joltage
            // Day 3 - Part 1 solution 17535 - elapsed 72.34µs
            max_joltage(bank, 2)
        })
        .sum()
}

pub fn part_2(input: &[&[u8]]) -> u64 {
    input.iter().map(|&bank| max_joltage(bank, 12)).sum()
}

/// Computes maximum 12-digit joltage from a battery bank using greedy selection.
///
/// # Algorithm
/// Selects exactly 12 batteries left-to-right to maximize the resulting decimal number.
/// For each digit position i ∈ [1, 12]:
/// - Search space: `bank[curr_start .. N - (12 - i)]`
/// - Select the leftmost maximum digit in this space
/// - Advance `curr_start` past the selected battery
///
/// # Correctness
/// Greedy choice is optimal: selecting the leftmost maximum for position i guarantees
/// no alternative selection can produce a larger number, since:
/// 1. Any larger digit at position i dominates all subsequent digit choices
/// 2. Among equal maxima, leftmost selection maximizes the search space for future digits
///
/// # Complexity
/// - Time: O(N) per bank, where N is battery count
/// - Space: O(1)
///
/// Each iteration scans a shrinking window; total work is O(N) across all 12 iterations.
/// Because windows are disjoint—each battery falls into at most one window.
fn max_joltage(bank: &[u8], n_digits: usize) -> u64 {
    let bank_len = bank.len();
    let mut max_joltage = 0;
    let mut curr_start = 0;

    for digits_remaining in (1..=n_digits).rev() {
        let must_reserve = digits_remaining - 1;
        let end = bank_len - must_reserve;

        let (max_digit, rel_max_idx) = left_most_max(&bank[curr_start..end]);

        max_joltage = max_joltage * 10 + u64::from(max_digit);
        curr_start += rel_max_idx + 1;
    }

    max_joltage
}

fn left_most_max(sub_bank: &[u8]) -> (u8, usize) {
    let mut left_most_max = (0, 0);
    for (idx, b) in sub_bank.iter().enumerate().map(|(idx, &b)| (idx, b - b'0')) {
        if b > left_most_max.0 {
            left_most_max = (b, idx);
        }
    }
    left_most_max
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

    #[test]
    fn test_part_2() {
        let input = parse(TEST_INPUT);
        assert_eq!(3121910778619, part_2(&input))
    }
}

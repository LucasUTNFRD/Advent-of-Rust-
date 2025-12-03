use rustc_hash::FxHashSet as HashSet;

pub fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|s| {
            let (a, b) = s.trim().split_once('-')?;
            Some((a.parse().ok()?, b.parse().ok()?))
        })
        .collect()
}

pub fn part_1(ids: &[(u64, u64)]) -> u64 {
    ids.iter().map(|&id_range| get_invalid_ids(id_range)).sum()
}

// 0 - 99 -> 11 22 33 44 55 66 .. 99
// 100 - 999
// 1000 - 9999 00 10   (HAS LEADING ZEROS SO IGNORE IT)  20

fn get_invalid_ids(id_range: (u64, u64)) -> u64 {
    let (start, end): (u64, u64) = (id_range.0, id_range.1);
    let start_len = if start == 0 { 1 } else { start.ilog10() + 1 };
    let end_len = end.ilog10() + 1;

    (start_len..=end_len)
        .filter(|x| x % 2 == 0)
        .flat_map(generate_repeated_sequences)
        .filter(|&n| n >= start && n <= end)
        .sum()
}

// any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
// previously i was returning Vec<u64> instead of iterator was .8 ms faster
fn generate_repeated_sequences(n: u32) -> impl Iterator<Item = u64> {
    let half_len = n / 2;

    let start = 10_u64.pow(half_len - 1);
    let end = 10_u64.pow(half_len);

    // Multiplier creates the repetition.
    // E.g., for n=4, half_len=2, multiplier = 100 + 1 = 101.
    // 12 * 101 = 1212.
    let multiplier = 10_u64.pow(half_len) + 1;

    (start..end).map(move |x| x * multiplier)
}

#[inline(never)]
pub fn part_2(ids: &[(u64, u64)]) -> u64 {
    ids.iter()
        .map(|&id_range| get_invalid_ids_part_2(id_range))
        .sum()
}

#[inline(never)]
fn get_invalid_ids_part_2(id_range: (u64, u64)) -> u64 {
    let (start, end): (u64, u64) = (id_range.0, id_range.1);

    let start_len = start.ilog10() + 1;
    let end_len = end.ilog10() + 1;

    let unique_invalid_ids: HashSet<u64> = (start_len..=end_len)
        .flat_map(generate_sequence_part_2)
        .filter(|&n| n >= start && n <= end)
        .collect(); // FIX: Remove this Collect as HashSet for this generate_sequence MUST NOT produce repeated sequences

    // 2. Sum the elements of the HashSet.
    unique_invalid_ids.into_iter().sum()
}

#[inline(never)]
fn generate_sequence_part_2(total_digits: u32) -> impl Iterator<Item = u64> {
    // Find all divisors of total_digits to get pattern lengths
    let divisors: Vec<u32> = (1..=total_digits)
        .filter(|&d| total_digits.is_multiple_of(d) && d < total_digits)
        .collect();

    divisors.into_iter().flat_map(move |pattern_len| {
        let repeats = total_digits / pattern_len;

        let pattern_start = 10_u64.pow(pattern_len - 1);
        let pattern_end = 10_u64.pow(pattern_len);

        (pattern_start..pattern_end).map(move |pattern| {
            let mut result = 0u64;
            let multiplier = 10u64.pow(pattern_len);
            for _ in 0..repeats {
                result = result * multiplier + pattern;
            }
            result
            // HOT AFFF
            // let pattern_str = pattern.to_string();
            // let repeated = pattern_str.repeat(repeats as usize);
            // repeated.parse::<u64>().unwrap()
        })
        // .collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod test {
    use crate::year2025::{self, day02::parse};

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_part_1() {
        let input = parse(TEST_INPUT);
        assert_eq!(1227775554, year2025::day02::part_1(&input))
    }

    #[test]
    fn test_part_2() {
        let input = parse(TEST_INPUT);
        assert_eq!(4174379265, year2025::day02::part_2(&input))
    }
}

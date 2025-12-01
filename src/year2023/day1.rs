use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));

            match (digits.next(), digits.next_back()) {
                (Some(first), Some(last)) => first * 10 + last,
                (None, Some(last)) => last * 10 + last,
                (Some(first), None) => first * 10 + first,
                (None, None) => unreachable!(),
            }
        })
        .sum()
}

pub fn part_2(input: &[&str]) -> u32 {
    input
        // using par_iter takes 25 ms less that iter
        .par_iter()
        .map(|line| {
            let mut digits = str_to_digits(line).into_iter();

            match (digits.next(), digits.next_back()) {
                (Some(first), Some(last)) => first * 10 + last,
                (None, Some(last)) => last * 10 + last,
                (Some(first), None) => first * 10 + first,
                (None, None) => unreachable!(),
            }
        })
        .sum()
}

const DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn get_digit_at(line: &str, i: usize) -> Option<u32> {
    let line_bytes = line.as_bytes();

    if let Some(&byte) = line_bytes.get(i)
        && byte.is_ascii_digit()
    {
        return Some((byte - b'0') as u32);
    }

    for (value, digit_pattern) in DIGITS.iter().enumerate() {
        if i + digit_pattern.len() <= line_bytes.len() && line_bytes[i..].starts_with(digit_pattern)
        {
            return Some(value as u32 + 1);
        }
    }

    None
}

fn str_to_digits(line: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        if let Some(digit) = get_digit_at(line, i) {
            digits.push(digit);
        }
    }

    digits
}

#[cfg(test)]
mod test {
    use crate::year2023::day1::{parse, part_1, part_2};

    const SAMPLE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const SAMPLE_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
    #[test]
    fn test_parse_input() {
        let val = part_1(&parse(SAMPLE_INPUT));
        assert_eq!(val, 142)
    }

    #[test]
    fn test_digit_to_u32() {
        let val = part_2(&parse(SAMPLE_INPUT_2));
        assert_eq!(val, 281)
    }
}

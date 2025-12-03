// Given that number are in range 0..99 we can avoid using a Hashet and using Static Bool Set
type Set = [bool; 100];

pub fn parse(input: &str) -> Vec<(Set, Set)> {
    input
        .lines()
        .map(|line| {
            let (winning_str, having_str) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let mut winning_mask = [false; 100];

            for n in winning_str
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
            {
                winning_mask[n] = true;
            }

            let mut having_mask = [false; 100];
            for n in having_str
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
            {
                having_mask[n] = true;
            }

            (winning_mask, having_mask)
        })
        .collect()
}

pub fn part_1(input: &[(Set, Set)]) -> u32 {
    input
        .iter()
        .map(|(winning, having)| {
            let mut match_count = 0;
            for k in 0..100 {
                // If the number 'k' is in BOTH sets, it's a match.
                if winning[k] && having[k] {
                    match_count += 1;
                }
            }
            if match_count == 0 {
                0
            } else {
                1 << (match_count - 1)
            }
        })
        .sum()
}

pub fn part_2(input: &[(Set, Set)]) -> u32 {
    let mut counts = vec![1; input.len()];
    for i in 0..input.len() {
        let (winning, having) = &input[i];

        let current_instances = counts[i];

        let mut match_count = 0;
        for k in 0..100 {
            // If the number 'k' is in BOTH sets, it's a match.
            if winning[k] && having[k] {
                match_count += 1;
            }
        }

        if match_count > 0 {
            let end_index = (i + match_count + 1).min(input.len());

            for counts in counts.iter_mut().take(end_index).skip(i + 1) {
                *counts += current_instances;
            }
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::year2023::day4::{parse, part_1};

    const SAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(14, part_1(&parse(SAMPLE_INPUT)));
    // }
}

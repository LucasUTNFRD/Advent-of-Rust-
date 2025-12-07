use std::ops::RangeInclusive;

pub struct Input {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}
pub fn parse(input: &str) -> Input {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .lines()
        .map(|range_l| {
            let (start, end) = range_l.split_once('-').unwrap();
            let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());
            (start, end)
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable();

    let mut merged_ranges = Vec::with_capacity(100);
    let mut curr_merged = ranges[0];

    for &next_range in ranges.iter().skip(1) {
        let curr_end = curr_merged.1;
        let next_start = next_range.0;

        if next_start <= curr_end + 1 {
            let new_end = curr_end.max(next_range.1);
            curr_merged = (curr_merged.0, new_end);
        } else {
            merged_ranges.push(curr_merged);
            curr_merged = next_range;
        }
    }
    merged_ranges.push(curr_merged);
    let ranges = merged_ranges;

    let ids = ids
        .lines()
        .map(|id| id.parse().unwrap())
        .collect::<Vec<_>>();

    Input { ranges, ids }
}
pub fn part_1(input: &Input) -> usize {
    input
        .ranges
        .iter()
        .map(|&(start, end)| {
            // Where does the range start
            let start_pos = input.ids.partition_point(|&id| id < start);
            // Where does the range end
            let end_pos = input.ids.partition_point(|&id| id <= end);
            end_pos - start_pos
        })
        .sum()
}

pub fn part_2(input: &Input) -> u64 {
    input
        .ranges
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum()
}

pub fn part_1(input: &str) -> u32 {
    let mut sections = input.split("\n\n");
    let seeds: Vec<u32> = sections
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let maps: Vec<Vec<(u32, u32, u32)>> = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let parts: Vec<u32> = line
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    (parts[0], parts[1], parts[2])
                })
                .collect()
        })
        .collect();

    seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |current_num, map| {
                for &(destination_start, source_start, length) in map {
                    if current_num >= source_start && current_num < source_start + length {
                        return destination_start + (current_num - source_start);
                    }
                }
                current_num
            })
        })
        .min()
        .unwrap_or(0)
}

pub fn part_2(input: &str) -> u32 {
    let mut sections = input.split("\n\n");

    let seeds: Vec<u32> = sections
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let maps: Vec<Vec<(u32, u32, u32)>> = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let parts: Vec<u32> = line
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    (parts[0], parts[1], parts[2])
                })
                .collect()
        })
        .collect();

    let mut current_ranges: Vec<_> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();

    for map_layer in maps {
        let mut next_stage_ranges: Vec<(u32, u32)> = Vec::new();

        let mut unprocessed = current_ranges;

        for (dest_start, src_start, length) in map_layer {
            let src_end = src_start + length;
            let mut next_unprocessed: Vec<(u32, u32)> = Vec::new();

            while let Some((start, end)) = unprocessed.pop() {
                // Calculate Intersection
                // Max of starts, Min of ends
                let overlap_start = start.max(src_start);
                let overlap_end = end.min(src_end);

                if overlap_start < overlap_end {
                    // WE HAVE AN OVERLAP!

                    // 1. Process the overlapping part (Transform it)
                    // Apply the map shift: (overlap - src) + dest
                    let offset = overlap_start - src_start;
                    let new_dest = dest_start + offset;
                    let new_len = overlap_end - overlap_start;

                    // Push to next_stage_ranges because it is DONE for this entire layer
                    next_stage_ranges.push((new_dest, new_dest + new_len));

                    // 2. Handle the non-overlapping parts (Left and Right leftovers)
                    // These go back into 'next_unprocessed' to be checked by other rules in this layer
                    if start < overlap_start {
                        next_unprocessed.push((start, overlap_start));
                    }
                    if end > overlap_end {
                        next_unprocessed.push((overlap_end, end));
                    }
                } else {
                    // NO OVERLAP
                    // Keep the whole range for the next rule check
                    next_unprocessed.push((start, end));
                }
            }
            unprocessed = next_unprocessed;
        }

        // Any ranges remaining in 'unprocessed' did not match ANY rule in this layer.
        // They pass through unchanged (Source number = Dest number).
        next_stage_ranges.extend(unprocessed);

        // Prepare for the next layer (e.g., move from Soil to Fertilizer)
        current_ranges = next_stage_ranges;
    }

    current_ranges
        .iter()
        .map(|(start, _)| *start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::year2023::day5;

    const SAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1() {
        assert_eq!(35, day5::part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(46, day5::part_2(SAMPLE_INPUT));
    }
}

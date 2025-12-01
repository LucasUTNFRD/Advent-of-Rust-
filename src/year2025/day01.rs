const DIAL_START: i32 = 50;
const N: i32 = 100;

pub fn part_1(input: &str) -> usize {
    // 0..99
    let mut dial: i32 = DIAL_START;
    let mut count = 0;
    for line in input.lines() {
        let (rotation, number) = line.split_at(1);
        let number = number.parse::<i32>().unwrap();
        dial = if rotation == "L" {
            dial - number
        } else {
            dial + number
        }
        .rem_euclid(N);

        count = usize::from(dial == 0);
    }

    count
}

pub fn part_2(input: &str) -> u32 {
    let mut dial: i32 = DIAL_START;
    let mut count = 0;

    for line in input.lines() {
        let (rotation, number) = line.split_at(1);

        let number = number.parse::<i32>().unwrap();

        if rotation == "R" {
            count += (dial + number).div_euclid(N);
            dial = (dial + number).rem_euclid(N)
        } else {
            // Calculate distance to 0 going Left
            // Example: Dial 10. Distance to 0 (Left) is 10.
            // Example: Dial 0. Distance to 0 (Left) is 0.

            // We hit 0 if number > dial.
            // (Strictly greater, because if number == dial, we land on 0, which counts).
            // Actually, if dial is 10 and we go L10, we land on 0. Count = 1.
            // If dial is 0 and we go L10, we go 0->90. Count = 0.

            // To handle "dial=0" case:
            // If dial is 0, effectively treat it as 100 for the subtraction logic,
            // OR just use the formula:

            let reversed_pos = if dial == 0 { N } else { dial };

            if number >= reversed_pos {
                count += 1 + (number - reversed_pos) / N;
            }

            dial = (dial - number).rem_euclid(N);
        }
    }

    count as u32
}

#[cfg(test)]
mod test {
    use crate::year2025;

    const SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_1() {
        assert_eq!(3, year2025::day01::part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, year2025::day01::part_2(SAMPLE_INPUT));
    }

    #[test]
    fn assert_r100_edge_case() {
        assert_eq!(10, year2025::day01::part_2("R1000"))
    }
}

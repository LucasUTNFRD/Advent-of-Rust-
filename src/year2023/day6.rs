//! * `x` is time spent holding the button.
//! * `t` is the duration of the race.
//! * `d` is the record distance.
//!
//! To beat the record the following condition must hold:
//!
//! * `x * (t - x) = d`
//! * `x² - tx +d = 0`

pub fn part_1(input: &str) -> u64 {
    let (time_line, dist_line) = input.split_once('\n').unwrap();

    let times = time_line
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok());

    let distances = dist_line
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok());

    times
        .zip(distances)
        .map(|(t, d)| different_ways(t, d))
        .product()
}

fn different_ways(t: u64, d: u64) -> u64 {
    let t_f = t as f64;
    let d_f = d as f64;

    let discriminant = t_f.powi(2) - 4.0 * d_f;

    if discriminant < 0.0 {
        return 0;
    }

    let sqrt_val = discriminant.sqrt();

    let r1 = (t_f - sqrt_val) / 2.0;
    let r2 = (t_f + sqrt_val) / 2.0;

    let lower_bound = r1.floor() as u64 + 1;

    let upper_bound = r2.ceil() as u64 - 1;

    if upper_bound < lower_bound {
        return 0;
    }

    (upper_bound - lower_bound) + 1
}

pub fn part_2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time_string: String = lines
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .collect();

    let time: u64 = time_string.parse().unwrap();

    let distances: String = lines
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .collect();

    let record_distance: u64 = distances.parse().unwrap();

    different_ways(time, record_distance)
}

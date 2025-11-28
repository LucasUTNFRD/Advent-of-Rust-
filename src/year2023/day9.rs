pub fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_1(oasis_report: &[Vec<i64>]) -> i64 {
    oasis_report.iter().map(|v| extrapolate_fordward(v)).sum()
}

fn extrapolate_fordward(history: &[i64]) -> i64 {
    let mut sequences = vec![history.to_vec()];

    while !sequences.last().unwrap().iter().all(|&v| v == 0) {
        let curr_seq = sequences.last().unwrap();
        let next_seq: Vec<i64> = curr_seq.windows(2).map(|w| w[1] - w[0]).collect();

        sequences.push(next_seq);
    }

    sequences.iter().map(|seq| seq.last().unwrap()).sum()
}

fn extrapolate_backward(history: &[i64]) -> i64 {
    let mut sequences = vec![history.to_vec()];

    while !sequences.last().unwrap().iter().all(|&v| v == 0) {
        let curr_seq = sequences.last().unwrap();
        let next_seq: Vec<i64> = curr_seq.windows(2).map(|w| w[1] - w[0]).collect();

        sequences.push(next_seq);
    }

    let _zeroes = sequences.pop();

    let mut extrapolation = 0;
    for curr_seq in sequences.iter().rev() {
        let first_val = *curr_seq.first().unwrap();
        extrapolation = first_val - extrapolation;
    }

    extrapolation
}

pub fn part_2(oasis_report: &[Vec<i64>]) -> i64 {
    oasis_report.iter().map(|v| extrapolate_backward(v)).sum()
}

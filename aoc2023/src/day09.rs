use aoc2021::data_str;
use aoc2021::utils::{split_lines, AdventError};


pub fn run() -> (i64, i64) {
    let input = data_str!("day09");
    let sequences = parse(input).expect("invalid input");

    (
        extrapolated_sum(&sequences),
        extrapolated_sum_left(&sequences),
    )
}


fn parse(input: &str) -> Result<Vec<Vec<i64>>, AdventError> {
    let lines = split_lines(input).iter()
        .map(
            |line| line.split_ascii_whitespace()
                .map(|s| s.trim().parse().map_err(AdventError::Parser))
                .collect()
        )
        .collect();

    lines
}

fn extrapolated_sum(sequences: &[Vec<i64>]) -> i64 {
    sequences.iter().map(|seq| extrapolate(seq)).sum()
}

fn extrapolated_sum_left(sequences: &[Vec<i64>]) -> i64 {
    sequences.iter().map(|seq| extrapolate_left(seq)).sum()
}

fn extrapolate(sequence: &[i64]) -> i64 {
    let mut differences: Vec<Vec<i64>> = vec![sequence.iter().cloned().collect()];

    loop {
        let d = diff(differences.last().unwrap());
        if d.len() == 0 {
            panic!()
        }
        if d.iter().all(|&i| i == 0) {
            break;
        }
        differences.push(d);
    }

    let mut unkowns: Vec<i64> = vec![0];
    for diffs in differences.iter().rev() {
        let d = unkowns.last().unwrap();
        let u = diffs.last().unwrap() + d;
        unkowns.push(u);
    }

    *unkowns.last().unwrap()
}

fn diff(seq: &[i64]) -> Vec<i64> {
    seq.iter()
        .skip(1)
        .zip(seq)
        .map(|(i, j)| i - j)
        .collect()
}

fn extrapolate_left(sequence: &[i64]) -> i64 {
    let mut differences: Vec<Vec<i64>> = vec![sequence.iter().cloned().collect()];

    loop {
        let d = diff(differences.last().unwrap());
        if d.len() == 0 {
            panic!()
        }
        if d.iter().all(|&i| i == 0) {
            break;
        }
        differences.push(d);
    }

    let mut unkowns: Vec<i64> = vec![0];
    for diffs in differences.iter().rev() {
        let d = unkowns.last().unwrap();
        let u = diffs.first().unwrap() - d;
        unkowns.push(u);
    }

    *unkowns.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ";

        let sequences = parse(input).expect("invalid input");
        assert_eq!(extrapolate(&sequences[0]), 18);
        assert_eq!(extrapolate(&sequences[1]), 28);
        assert_eq!(extrapolate(&sequences[2]), 68);
        assert_eq!(extrapolated_sum(&sequences), 114);

        assert_eq!(extrapolate_left(&sequences[0]), -3);
        assert_eq!(extrapolate_left(&sequences[1]), 0);
        assert_eq!(extrapolate_left(&sequences[2]), 5);
        assert_eq!(extrapolated_sum_left(&sequences), 2);
    }
}
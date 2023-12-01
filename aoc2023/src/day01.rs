use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};

pub fn run() -> (u32, u32) {

    let input = data_str!("day01");
    let data = parse(input).expect("invalid input");

    (
        sum_of_calibration_values(&data),
        0
    )
}

fn sum_of_calibration_values(values: &[(u32, u32)]) -> u32 {
    values.iter().map(|(x, y)| x * 10 + y).sum()
}

fn parse(input: &str) -> Result<Vec<(u32, u32)>, AdventError> {
    split_lines(input).iter()
        .map(|line| {
            let mut matches = line.matches(|c: char| c.is_ascii_digit());
            let first = matches.next().unwrap();
            let second = matches.next_back().unwrap_or(first);
            Ok((first.parse()?, second.parse()?))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";

        let data = parse(input).expect("invalid input");

        assert_eq!(sum_of_calibration_values(&data), 142);
    }
}
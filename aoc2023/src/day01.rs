use itertools::Itertools;

use aoc2021::data_str;
use aoc2021::utils::AdventError;

pub fn run() -> (u32, u32) {

    let input = data_str!("day01");
    let data = parse(input).expect("invalid input");

    (
        0,
        0
    )
}

fn sum_of_calibration_values(values: &[(u32, u32)]) {

}

fn parse(input: &str) -> Result<Vec<(u32, u32)>, AdventError> {
    split_lines(input).iter()
        .map(|block| {
            block.trim()
                .split('\n')
                .map(|line| line.trim().parse::<u32>().map_err(AdventError::Parser))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

    }
}
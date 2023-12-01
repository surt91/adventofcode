use std::collections::HashMap;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};

pub fn run() -> (u32, u32) {

    let input = data_str!("day01");
    let data = parse(input, false).expect("invalid input");
    let data2 = parse(input, true).expect("invalid input");

    (
        sum_of_calibration_values(&data),
        sum_of_calibration_values(&data2),
    )
}

fn sum_of_calibration_values(values: &[(u32, u32)]) -> u32 {
    values.iter().map(|(x, y)| x * 10 + y).sum()
}

fn parse(input: &str, spelled: bool) -> Result<Vec<(u32, u32)>, AdventError> {
    let mut values = HashMap::new();
    if spelled {
        values.insert("one", 1);
        values.insert("two", 2);
        values.insert("three", 3);
        values.insert("four", 4);
        values.insert("five", 5);
        values.insert("six", 6);
        values.insert("seven", 7);
        values.insert("eight", 8);
        values.insert("nine", 9);
    }
    values.insert("1", 1);
    values.insert("2", 2);
    values.insert("3",3);
    values.insert("4", 4);
    values.insert("5", 5);
    values.insert("6", 6);
    values.insert("7", 7);
    values.insert("8", 8);
    values.insert("9", 9);

    let digits: Vec<&str> = values.keys().cloned().collect();

    split_lines(input).iter()
        .map(|line| {
            let left = digits.iter()
                .filter_map(
                    |digit| line.find(digit).map(|idx| (idx, digit))
                )
                .min()
                .ok_or(AdventError::NotEnoughElements)?;
            let right = digits.iter()
                .filter_map(
                    |digit| line.rfind(digit).map(|idx| (idx, digit))
                )
                .max()
                .ok_or(AdventError::NotEnoughElements)?;
            Ok((values[left.1], values[right.1]))
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

        let data = parse(input, false).expect("invalid input");

        assert_eq!(sum_of_calibration_values(&data), 142);

        let input = r"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";

        let data = parse(input, true).expect("invalid input");

        assert_eq!(sum_of_calibration_values(&data), 281);
    }
}
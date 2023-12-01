use core::num;
use std::collections::HashMap;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};

pub fn run() -> (u32, u32) {

    let input = data_str!("day01");
    let data = parse(input).expect("invalid input");
    let data2 = parse2(input).expect("invalid input");

    (
        sum_of_calibration_values(&data),
        sum_of_calibration_values(&data2),
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

fn parse2(input: &str) -> Result<Vec<(u32, u32)>, AdventError> {
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut values = HashMap::new();
    values.insert("one", 1);
    values.insert("two", 2);
    values.insert("three", 3);
    values.insert("four", 4);
    values.insert("five", 5);
    values.insert("six", 6);
    values.insert("seven", 7);
    values.insert("eight", 8);
    values.insert("nine", 9);
    values.insert("1", 1);
    values.insert("2", 2);
    values.insert("3",3);
    values.insert("4", 4);
    values.insert("5", 5);
    values.insert("6", 6);
    values.insert("7", 7);
    values.insert("8", 8);
    values.insert("9", 9);

    split_lines(input).iter()
        .map(|line| {
            let left = digits.iter()
                .map(|digit| (line.find(digit), digit))
                .filter(|i| i.0.is_some())
                .min()
                .unwrap();
            let right = digits.iter()
                .map(|digit| (line.rfind(digit), digit))
                .filter(|i| i.0.is_some())
                .max()
                .unwrap();
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

        let data = parse(input).expect("invalid input");

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

        let data = parse2(input).expect("invalid input");
        println!("{:?}", data);

        assert_eq!(sum_of_calibration_values(&data), 281);
    }
}
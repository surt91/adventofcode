use std::{fs, str::FromStr, num::ParseIntError};

use super::AdventError;

pub fn read_lines(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("input file does not exist");
    split_lines(&input)
}

pub fn split_lines(input: &str) -> Vec<String> {
    input.split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|str| str.to_string())
        .collect()
}

pub fn parse_single_line<T>(input: &str) -> Result<Vec<T>, AdventError>
    where T: FromStr<Err = ParseIntError>
{
    input.split(',')
        .map(|i|
            i.parse()
                .map_err(AdventError::Parser)
        )
        .collect()
}
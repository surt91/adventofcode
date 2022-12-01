use std::{str::FromStr, num::ParseIntError};


use super::AdventError;

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
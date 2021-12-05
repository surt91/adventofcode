use std::{fs, fmt, error::Error};

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

#[derive(Debug, Clone)]
pub struct InvalidInput;

impl fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input")
    }
}

impl Error for InvalidInput {}

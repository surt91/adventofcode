use itertools::Itertools;

use aoc2021::data_str;
use aoc2021::utils::AdventError;

pub fn run() -> (u32, u32) {

    let input = data_str!("day01");
    let data = parse(input).expect("invalid input");

    (
        elf_with_most_calories(&data),
        top_3_with_most_calories(&data),
    )
}

fn elf_with_most_calories(data: &[Vec<u32>]) -> u32  {
    data.iter()
        .map(|elf| elf.iter().sum::<u32>())
        .max()
        .unwrap()
}

fn top_3_with_most_calories(data: &[Vec<u32>]) -> u32  {
    data.iter()
        .map(|elf| elf.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum::<u32>()
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, AdventError> {
    input.trim()
        .split("\n\n")
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
        let input = r"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        ";

        let data = parse(input).expect("invalid input");

        assert_eq!(elf_with_most_calories(&data), 24000);
        assert_eq!(top_3_with_most_calories(&data), 45000);
    }
}
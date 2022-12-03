use std::collections::HashSet;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};
use itertools::Itertools;

pub fn run() -> (u32, u32) {

    let input = data_str!("day03");
    let data = parse(input);
    let data2 = parse2(input);

    (
        priority_sum(&data),
        group_sum(&data2)
    )
}

fn priority_sum(rucksacks: &[(HashSet<char>, HashSet<char>)]) -> u32 {
    rucksacks.iter()
        .flat_map(|(first, second)| first.intersection(second).next())
        .flat_map(|item| priority(*item))
        .sum()
}

fn group_sum(elfs: &[HashSet<char>]) -> u32 {
    elfs.iter()
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| chunk.cloned().collect_tuple())
        .map(badge)
        .flat_map(priority)
        .sum()
}

fn priority(item: char) -> Result<u32, AdventError> {
    println!("{} -> {}", item, if item.is_ascii_lowercase() {item as u32 - 96} else {item as u32 - 64});

    if item.is_ascii_lowercase() {
        Ok(item as u32 - 96)
    } else if item.is_ascii_uppercase() {
        Ok(item as u32 - 64 + 26)
    } else {
        Err(AdventError::UnexpectedElement { found: item.to_string(), expected: &["a-zA-Z"] })
    }
}

fn badge((elf1, elf2, elf3): (HashSet<char>, HashSet<char>, HashSet<char>)) -> char {
    *elf1.intersection(&elf2)
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&elf3)
        .next()
        .unwrap()
}

fn parse(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    split_lines(input).iter()
        .map(|line| {
            let length = line.len();
            let compartement_size = length / 2;
            let first_half = line.chars().take(compartement_size).collect();
            let second_half = line.chars().skip(compartement_size).collect();
            (first_half, second_half)
        })
        .collect()
}

fn parse2(input: &str) -> Vec<HashSet<char>> {
    split_lines(input).iter()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        let data = parse(input);
        let data2 = parse2(input);

        assert_eq!(priority_sum(&data), 157);
        assert_eq!(group_sum(&data2), 70);
    }
}
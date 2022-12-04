use bit_set::BitSet;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};
use itertools::Itertools;

pub fn run() -> (usize, usize) {

    let input = data_str!("day03");
    let data = parse(input);
    let data2 = parse2(input);

    (
        priority_sum(&data),
        group_sum(&data2)
    )
}

fn priority_sum(rucksacks: &[(BitSet, BitSet)]) -> usize {
    rucksacks.iter()
        .flat_map(|(first, second)| first.intersection(second).next())
        .flat_map(priority)
        .sum()
}

fn group_sum(elfs: &[BitSet]) -> usize {
    elfs.iter()
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| chunk.cloned().collect_tuple())
        .map(badge)
        .flat_map(priority)
        .sum()
}

fn priority(item: usize) -> Result<usize, AdventError> {
    match (item as u8) as char {
        ch if ch.is_ascii_lowercase() => Ok(item - 96),
        ch if ch.is_ascii_uppercase() => Ok(item - 64 + 26),
        _ => Err(AdventError::UnexpectedElement { found: item.to_string(), expected: &["a-zA-Z"] }),
    }
}

fn badge((mut elf1, elf2, elf3): (BitSet, BitSet, BitSet)) -> usize {
    elf1.intersect_with(&elf2);
    elf1.intersect_with(&elf3);
    elf1.iter().next().unwrap()
}

fn parse(input: &str) -> Vec<(BitSet, BitSet)> {
    split_lines(input).iter()
        .map(|line| {
            let length = line.len();
            let compartment_size = length / 2;
            let first_half = line.as_bytes()
                .iter()
                .take(compartment_size)
                .map(|&x| x.into())
                .collect();
            let second_half = line.as_bytes()
                .iter()
                .skip(compartment_size)
                .map(|&x| x.into())
                .collect();
            (first_half, second_half)
        })
        .collect()
}

fn parse2(input: &str) -> Vec<BitSet> {
    split_lines(input).iter()
        .map(|line| BitSet::from_iter(
            line.as_bytes()
                .iter()
                .map(|&x| x.into())
        ))
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
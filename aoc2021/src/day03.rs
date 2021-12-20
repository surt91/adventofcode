use std::cmp::Ordering;

use crate::utils::{read_lines, binary};

pub fn run() -> (usize, usize) {
    let diagnostics = read_lines("data/day03a.dat");

    (
        power_consumption(&diagnostics),
        life_support(&diagnostics)
    )
}

fn power_consumption(diagnostics: &[String]) -> usize {
    gamma(diagnostics) * epsilon(diagnostics)
}

fn strings_to_vec(strings: &[String]) -> Vec<Vec<u8>> {
    strings.iter().map(|line|
        line.chars()
            .map(|c| c.to_digit(10)
                .unwrap() as u8
            ).collect()
    ).collect()
}

fn binary(lines: &[Vec<u8>], tiebreaker: u8) -> Vec<u8> {
    let num_lines = lines.len();
    let num_columns = lines[0].len();

    (0..num_columns).map(|i| {
        let num_ones: usize = lines.iter()
            .map(|line| line[i] as usize)
            .sum();

        match (2 * num_ones).cmp(&num_lines) {
            Ordering::Greater => 1,
            Ordering::Less => 0,
            Ordering::Equal => tiebreaker
        }
    }).collect()
}

fn gamma(lines: &[String]) -> usize {
    let data = strings_to_vec(lines);
    let bin = binary(&data, 1);
    binary::to_usize(&bin)
}

fn epsilon(lines: &[String]) -> usize {
    let data = strings_to_vec(lines);
    let bin = binary(&data, 1);

    let inverted: Vec<u8> = bin.iter()
        .map(|&i| if i == 0 { 1 } else { 0 })
        .collect();

    binary::to_usize(&inverted)
}

fn criteria(lines: &[String], most: bool) -> Vec<u8> {
    let mut candidates = strings_to_vec(lines);

    let mut idx = 0;
    loop {
        if candidates.len() == 1 {
            return candidates[0].clone()
        }

        let most_common = binary(&candidates, 1)[idx];
        let least_common = if most_common == 1 { 0 } else { 1 };

        candidates.retain(|candidate|
            if most {
                candidate[idx] == most_common
            } else {
                candidate[idx] == least_common
            }
        );

        idx += 1;
    }
}

fn oxygen(lines: &[String]) -> usize {
    let sol = criteria(lines, true);
    binary::to_usize(&sol)
}

fn co2(lines: &[String]) -> usize {
    let sol = criteria(lines, false);
    binary::to_usize(&sol)
}

fn life_support(lines: &[String]) -> usize {
    oxygen(lines) * co2(lines)
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example() {
        let input = r"
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        ";

        let diagnostics = split_lines(input);

        assert_eq!(gamma(&diagnostics), 22);
        assert_eq!(epsilon(&diagnostics), 9);
        assert_eq!(power_consumption(&diagnostics), 198);

        assert_eq!(oxygen(&diagnostics), 23);
        assert_eq!(co2(&diagnostics), 10);
        assert_eq!(life_support(&diagnostics), 230);
    }
}

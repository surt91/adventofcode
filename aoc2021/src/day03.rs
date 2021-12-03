use std::cmp::Ordering;

use super::utils::read_lines;

pub fn run() {
    let diagnostics = read_lines("data/day03a.dat");

    println!("{}", power_consumption(&diagnostics));
    println!("{}", life_support(&diagnostics));
}

fn power_consumption(diagnostics: &[String]) -> i64 {
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

fn vec_to_int(vec: &[u8]) -> i64 {
    let bin: String = vec.iter().map(|c| format!("{}", c)).collect();
    i64::from_str_radix(&bin, 2).unwrap()
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

fn gamma(lines: &[String]) -> i64 {
    let data = strings_to_vec(lines);
    let bin = binary(&data, 1);
    vec_to_int(&bin)
}

fn epsilon(lines: &[String]) -> i64 {
    let data = strings_to_vec(lines);
    let bin = binary(&data, 1);

    let inverted: Vec<u8> = bin.iter()
        .map(|&i| if i == 0 { 1 } else { 0 })
        .collect();

    vec_to_int(&inverted)
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

        candidates = candidates.into_iter()
            .filter(|candidate|
                if most {
                    candidate[idx] == most_common
                } else {
                    candidate[idx] == least_common
                }
            ).collect();

        idx += 1;
    }
}

fn oxygen(lines: &[String]) -> i64 {
    let sol = criteria(lines, true);
    vec_to_int(&sol)
}

fn co2(lines: &[String]) -> i64 {
    let sol = criteria(lines, false);
    vec_to_int(&sol)
}

fn life_support(lines: &[String]) -> i64 {
    oxygen(lines) * co2(lines)
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example1() {
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

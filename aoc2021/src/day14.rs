use std::{fs, str::FromStr, collections::HashMap};

use itertools::Itertools;

use crate::utils::AdventError;

pub fn run() -> (isize, usize) {
    let input = fs::read_to_string("data/day14a.dat").expect("input file does not exist");
    let (template, rules) = parse(&input).expect("invalid input");

    (
        calculate(&template, &rules, 10),
        0
    )
}

fn calculate(template: &str, rules: &[Rule], iterations: usize) -> isize {
    let mut result = template.chars().collect_vec();
    let rule_map: HashMap<(char, char), char> = rules.iter()
        .map(|r| (r.pattern, r.insertion))
        .collect();

    for _ in 0..iterations {
        let mut idx = 0;
        while idx + 1 < result.len() {
            let pattern = (result[idx], result[idx+1]);
            let insertion = rule_map[&pattern];
            result.insert(idx+1, insertion);
            idx += 2
        }
    }

    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in result {
        *counter.entry(c).or_insert(0) += 1;
    }

    let min = *counter.values().min().unwrap();
    let max = *counter.values().max().unwrap();

    max as isize - min as isize
}

struct Rule {
    pattern: (char, char),
    insertion: char
}

impl FromStr for Rule {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        let mut it = line.trim().split("->");

        let mut p_it = it.next().ok_or(AdventError::NotEnoughElements)?.trim().chars();
        let p1 = p_it.next().ok_or(AdventError::NotEnoughElements)?;
        let p2 = p_it.next().ok_or(AdventError::NotEnoughElements)?;

        let insertion: char = it.next()
            .ok_or(AdventError::NotEnoughElements)?
            .trim()
            .chars()
            .next()
            .ok_or(AdventError::NotEnoughElements)?;

        Ok(Rule {
            pattern: (p1, p2),
            insertion
        })
    }
}

fn parse(input: &str) -> Result<(String, Vec<Rule>), AdventError> {
    let mut blocks = input.trim().split("\n\n");

    let template = blocks.next()
        .ok_or(AdventError::NotEnoughElements)?
        .trim()
        .to_string();

    let rules = blocks.next()
        .ok_or(AdventError::NotEnoughElements)?
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok((template, rules))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        ";

        let (template, rules) = parse(input).expect("invalid input");

        assert_eq!(calculate(&template, &rules, 10), 1588);
    }
}

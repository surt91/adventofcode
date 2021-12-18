use std::{fs, str::FromStr, collections::HashMap};

use itermore::IterMore;
use itertools::Itertools;

use crate::utils::AdventError;

pub fn run() -> (isize, isize) {
    let input = fs::read_to_string("data/day14a.dat").expect("input file does not exist");
    let (template, rules) = parse(&input).expect("invalid input");

    (
        calculate(&template, &rules, 10),
        calculate(&template, &rules, 40),
    )
}

fn calculate(template: &str, rules: &[Rule], iterations: usize) -> isize {
    let mut result: HashMap<(char, char), isize> = HashMap::new();

    for pair in template.chars()
        .windows()
        .map(|w: [char; 2]| (w[0], w[1]))
    {
        *result.entry(pair).or_insert(0) += 1;
    }

    let rule_map: HashMap<(char, char), char> = rules.iter()
        .map(|r| (r.pattern, r.insertion))
        .collect();

    for _ in 0..iterations {
        let to_insert = result.iter()
            .flat_map(|(&pair, &count)| {
                let c = rule_map[&pair];
                [
                    ((pair.0, c), count),
                    ((c, pair.1), count),
                    (pair, -count),
                ]
            })
            .collect_vec();

        for (pair, count) in to_insert {
            *result.entry(pair).or_insert(0) += count;
        }
    }

    let mut counter: HashMap<char, isize> = HashMap::new();
    for ((c1, _c2), count) in result {
        *counter.entry(c1).or_insert(0) += count;
        // only count the first one, to avoid counting twice
        // *counter.entry(_c2).or_insert(0) += count;
    }
    // then insert the last element of the template (which is never the first element)
    *counter.entry(template.chars().last().unwrap()).or_insert(0) += 1;

    let min = *counter.values().min().unwrap();
    let max = *counter.values().max().unwrap();

    max - min
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
        assert_eq!(calculate(&template, &rules, 40), 2188189693529);
    }
}

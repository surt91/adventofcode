use std::collections::HashMap;

use counter::Counter;
use scan_fmt::scan_fmt;

use aoc2021::data_str;
use aoc2021::utils::AdventError;

use crate::utils::factorize::factorize;


pub fn run() -> (usize, usize) {
    let input = data_str!("day08");
    let (map, instructions) = parse(input).expect("invalid input");

    (
        count_steps(map.clone(), instructions.clone()),
        count_ghost_steps(map, instructions),
    )
}

#[derive(Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Clone, Debug, Hash)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    fn new(label: String, left: String, right: String) -> Self {
        Node {
            label,
            left,
            right
        }
    }
}

fn parse(input: &str) -> Result<(HashMap<String, Node>, Vec<Instruction>), AdventError> {
    let mut map = HashMap::new();
    let mut it = input.trim().lines();
    let instructions = it.next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| {
            match c {
                'L' => Ok(Instruction::Left),
                'R' => Ok(Instruction::Right),
                _ => Err(AdventError::UnexpectedElement { found: c.to_string(), expected: &["R", "L"] })
            }
        })
        .collect::<Result<_, AdventError>>()?;
    it.next();

    for line in it {
        let (label, left, right) = scan_fmt!(
            line.trim(),
            "{} = ({}, {})",
            String, String, String
        )?;
        map.insert(label.clone(), Node::new(label, left, right));
    }

    Ok((map, instructions))
}

fn count_steps(map: HashMap<String, Node>, instructions: Vec<Instruction>) -> usize {
    let mut position = "AAA";
    let mut ctr = 0;
    loop {
        let direction = &instructions[ctr % instructions.len()];
        position = match direction {
            Instruction::Left => map[position].left.as_str(),
            Instruction::Right => map[position].right.as_str(),
        };
        ctr += 1;
        if position == "ZZZ" {
            break;
        }
    }
    ctr
}


fn count_ghost_steps(map: HashMap<String, Node>, instructions: Vec<Instruction>) -> usize {
    let mut positions: Vec<&str> = map.keys()
        .filter(|label| label.ends_with('A'))
        .map(|s| s.as_str())
        .collect();
    let mut ctr = 0;
    let mut loop_lengths = vec![0; positions.len()];
    loop {
        let mut finished = true;
        let direction = &instructions[ctr % instructions.len()];
        for n in 0..positions.len() {
            if loop_lengths[n] > 0 {
                continue;
            }

            positions[n] = match direction {
                Instruction::Left => map[positions[n]].left.as_str(),
                Instruction::Right => map[positions[n]].right.as_str(),
            };

            if loop_lengths[n] == 0 && positions[n].ends_with('Z') {
                loop_lengths[n] = ctr + 1;
            }
            finished &= loop_lengths[n] > 0;
        }
        ctr += 1;
        if finished {
            break;
        }
    }
    least_common_multiple(loop_lengths)
}

fn least_common_multiple(numbers: Vec<usize>) -> usize {
    let mut prime_factors: Counter<usize> = Counter::new();
    for num in numbers {
        let factors = factorize(num);
        let factors: Counter<usize> = factors.into_iter().collect();
        for (prime, &multiplicity) in factors.iter() {
            if prime_factors[prime] < multiplicity {
                prime_factors[prime] = multiplicity;
            }
        }
    }

    prime_factors.iter()
        .map(|(prime, multiplicity)| prime * multiplicity)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        ";

        let (map, instructions) = parse(input).expect("invalid input");
        assert_eq!(count_steps(map, instructions), 2);

        let input = r"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";

        let (map, instructions) = parse(input).expect("invalid input");
        assert_eq!(count_steps(map, instructions), 6);

        let input = r"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";

        let (map, instructions) = parse(input).expect("invalid input");
        assert_eq!(count_ghost_steps(map, instructions), 6);

    }
}
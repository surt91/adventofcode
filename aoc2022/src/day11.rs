use std::{str::FromStr, fmt::Debug};

use aoc2021::{data_str, utils::{AdventError, split_lines}};
use itertools::Itertools;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    target: Box<dyn Fn(usize) -> usize>,
    divisor: usize,

    inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Box<dyn Fn(usize) -> usize>,
        target: Box<dyn Fn(usize) -> usize>,
        divisor: usize) -> Monkey
    {
        Monkey {
            items,
            operation,
            target,
            divisor,
            inspections: 0,
        }
    }
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            items: Default::default(),
            operation: Box::new(|x| x),
            target: Box::new(|x| x),
            divisor: Default::default(),
            inspections: Default::default()
        }
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspections", &self.inspections)
            .field("divisor", &self.divisor)
            .finish()
    }
}

impl FromStr for Monkey {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = split_lines(s);
        let mut lines = lines.iter();
        lines.next();
        let items: Vec<usize> = lines.next()
            .ok_or(AdventError::NotEnoughElements)?
            .split(':')
            .last()
            .ok_or(AdventError::NotEnoughElements)?
            .trim()
            .split(',')
            .map(|num| num.trim().parse().map_err(AdventError::Parser))
            .collect::<Result<_, AdventError>>()?;
        let operands = lines.next()
            .ok_or(AdventError::NotEnoughElements)?
            .split('=')
            .last()
            .ok_or(AdventError::NotEnoughElements)?
            .trim()
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect_vec();
        let operation = move |x: usize| {
            let op1 = match operands[0].as_str() {
                "old" => x,
                y => y.parse::<usize>().unwrap()
            };
            let op2 = match operands[2].as_str() {
                "old" => x,
                y => y.parse::<usize>().unwrap()
            };
            match operands[1].as_str() {
                "+" => op1 + op2,
                "*" => op1 * op2,
                _ => panic!("invalid operator: {operands:?}")
            }
        };
        let divisor: usize = lines.next()
            .ok_or(AdventError::NotEnoughElements)?
            .split(' ')
            .last()
            .ok_or(AdventError::NotEnoughElements)?
            .parse()?;
        let target_true = lines.next()
            .ok_or(AdventError::NotEnoughElements)?
            .split(' ')
            .last()
            .ok_or(AdventError::NotEnoughElements)?
            .parse()?;
        let target_false = lines.next()
            .ok_or(AdventError::NotEnoughElements)?
            .split(' ')
            .last()
            .ok_or(AdventError::NotEnoughElements)?
            .parse()?;
        let target = move |x| if x % divisor == 0 {target_true} else {target_false};

        Ok(Monkey::new(items, Box::new(operation), Box::new(target), divisor))
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day11");
    let mut data1: Vec<Monkey> = parse(input).expect("invalid input");
    let mut data2: Vec<Monkey> = parse(input).expect("invalid input");

    (
        monkey_business_level(&mut data1, 20, false),
        monkey_business_level(&mut data2, 10000, true),
    )
}

fn monkey_business_level(monkeys: &mut [Monkey], num_rounds: usize, ridiculous_worry_levels: bool) -> usize {
    for _i in 0..num_rounds {
        round(monkeys, ridiculous_worry_levels);
    }

    monkeys.iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn round(monkeys: &mut [Monkey], ridiculous_worry_levels: bool) {
    // addition and multiplication with modulo is distributive.
    // in order for all modulos of all monkeys to work, we can multiply them together.
    let magic_number: usize = monkeys.iter()
        .map(|m| m.divisor)
        .product();

    for i in 0..monkeys.len() {
        monkeys[i].inspections += monkeys[i].items.len();
        let monkey = std::mem::take(&mut monkeys[i]);
        for &item in &monkey.items {
            let mut new = (monkey.operation)(item);

            if ridiculous_worry_levels {
                new %= magic_number;
            } else {
                new /= 3;
            }

            let target = (monkey.target)(new);
            monkeys[target].items.push(new);
        }
        monkeys[i] = monkey;
        monkeys[i].items.clear()
    }
}

fn parse(input: &str) -> Result<Vec<Monkey>, AdventError> {
    input.split("\n\n")
        .map(|block| block.trim().parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            Monkey 0:
                Starting items: 79, 98
                Operation: new = old * 19
                Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
                Starting items: 54, 65, 75, 74
                Operation: new = old + 6
                Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
                Starting items: 79, 60, 97
                Operation: new = old * old
                Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
                Starting items: 74
                Operation: new = old + 3
                Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
        ";

        let mut data: Vec<Monkey> = parse(input).expect("invalid input");
        assert_eq!(monkey_business_level(&mut data, 20, false), 10605);

        let mut data: Vec<Monkey> = parse(input).expect("invalid input");
        assert_eq!(monkey_business_level(&mut data, 1, true), 24);

        let mut data: Vec<Monkey> = parse(input).expect("invalid input");
        assert_eq!(monkey_business_level(&mut data, 20, true), 99*103);

        let mut data: Vec<Monkey> = parse(input).expect("invalid input");
        assert_eq!(monkey_business_level(&mut data, 10000, true), 2713310158);

    }
}
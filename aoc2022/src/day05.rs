use itertools::Itertools;
use scan_fmt::scan_fmt;

use aoc2021::data_str;
use aoc2021::utils::split_lines;

type Instruction = (usize, usize, usize);

pub fn run() -> (String, usize) {

    let input = data_str!("day05");
    let (stacks, instructions) = parse(input);

    (
        find_top_crates(stacks, instructions),
        0
    )
}

fn find_top_crates(mut stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for (num, from, to) in instructions {
        println!("{:?}", stacks);
        println!("{} from {} to {}", num, from, to);
        for _ in 0..num {
            println!(".");
            let chest = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(chest)
        }
    }

    stacks.iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}


fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut parts = input.split("\n\n");
    let crates: Vec<String> = parts.next()
        .unwrap()
        .split('\n')
        .map(|str| str.to_string())
        .collect();
    let instructions = split_lines(parts.next().unwrap());

    let num_stacks = crates.iter()
        .rev()
        .next()
        .unwrap()
        .split_whitespace()
        .count();
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in crates.iter().rev().skip(1) {
        line.chars()
            .chunks(4)
            .into_iter()
            .enumerate()
            .for_each(|(n, mut chunk)| {
                let letter = chunk.nth(1).unwrap();
                if !letter.is_whitespace() {
                    stacks[n].push(letter);
                }
            });
    }

    let crane_commands = instructions.iter()
        .map(|line|
            scan_fmt!(
                line,
                "move {} from {} to {}",
                usize, usize, usize
            )
            .unwrap()
        )
        .collect();

    (stacks, crane_commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input =
r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    let (stacks, instructions) = parse(input);

        assert_eq!(find_top_crates(stacks, instructions), "CMZ");
    }
}
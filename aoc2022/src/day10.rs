use std::{str::FromStr, iter};

use aoc2021::{data_str, utils::{AdventError, split_lines, letters}};

enum Instruction {
    Addx(isize),
    Noop
}

impl FromStr for Instruction {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Instruction::Noop)
        } else if s.starts_with("addx") {
            let value = s.split(' ')
                .last()
                .ok_or(AdventError::WrongNumberOfElements)?
                .parse()?;
            Ok(Instruction::Addx(value))
        } else {
            Err(AdventError::UnexpectedElement { found: s.to_string(), expected: &["noop", "addx {}"] })
        }
    }
}

struct Registers {
    x: isize,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            x: 1
        }
    }

    fn trace(&mut self, instructions: &[Instruction]) -> Vec<isize> {
        // states at the start of each cycle
        let mut states = vec![0, self.x]; // start with 0, for 1-based indexing
        for i in instructions {
            match i {
                Instruction::Noop => states.push(self.x),
                Instruction::Addx(delta) => {
                    states.push(self.x);
                    self.x += delta;
                    states.push(self.x);
                }
            }
        }
        states
    }
}

pub fn run() -> (isize, String) {

    let input = data_str!("day10");
    let program: Vec<Instruction> = parse(input).expect("invalid input");
    let cycles = [20, 60, 100, 140, 180, 220];

    (
        signal_strengths(&cycles, &program),
        letters(&program)
    )
}

fn signal_strengths(cycles: &[usize], program: &[Instruction]) -> isize {
    let states = Registers::new().trace(program);

    cycles.iter()
        .map(|cycle|
            *cycle as isize * states[*cycle]
        )
        .sum()
}

fn raster(trace: &[isize]) -> Vec<bool> {
    trace.iter()
        .skip(1) // pay for 1-based indexing
        .take(240)
        .enumerate()
        .map(|(n, x)| n as isize % 40 >= x - 1 && n as isize % 40 <= x + 1)
        .collect()
}

fn _draw(program: &[Instruction]) -> String {
    let trace = Registers::new().trace(program);
    raster(&trace).iter()
        .map(|&p| if p {'#'} else {'.'})
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 40 == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect()
}

fn letters(program: &[Instruction]) -> String {
    let width = 40;
    let height = 6;
    let trace = Registers::new().trace(program);
    let image = raster(&trace);
    let mut transposed = vec![vec![vec!['\n'; height]; 5]; 8];
    for (n, &i) in image.iter().enumerate() {
        let (x, y) = (n % width, n / width);
        transposed[x/5][x%5][y] = if i {'#'} else {' '}
    }

    let transposed = transposed.iter().map(|letter|
        letter.iter()
            .take(4)
            .flat_map(|row|
                row[..=row.iter().rposition(|&c| c != ' ').unwrap_or(0)]
                    .iter()
                    .chain(iter::once(&'\n'))
            )
            .collect::<String>()
    );

    transposed
        .map(|letter| letters::parse(&letter).unwrap())
        .collect()
}

fn parse(input: &str) -> Result<Vec<Instruction>, AdventError> {
    split_lines(input).iter()
        .map(|line| line.parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            noop
            addx 3
            addx -5
        ";

        let program: Vec<Instruction> = parse(input).expect("invalid input");
        let trace = Registers::new().trace(&program);
        assert_eq!(trace, [0, 1, 1, 1, 4, 4, -1]);

        let input = r"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        ";

        let program: Vec<Instruction> = parse(input).expect("invalid input");
        let cycles = [20, 60, 100, 140, 180, 220];

        assert_eq!(signal_strengths(&cycles, &program), 13140);

        let expected_image =
r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(_draw(&program), expected_image);
    }
}
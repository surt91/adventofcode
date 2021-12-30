use std::{fs, str::FromStr};

use itertools::Itertools;
use rayon::prelude::*;

use crate::utils::AdventError;

const DIGITS: usize = 14;

// it's Christmas, I will just let my computer brute force this one...
pub fn run() -> (isize, isize) {
    let input = fs::read_to_string("data/day24a.dat").expect("input file does not exist");
    let alu: Alu = input.parse().expect("invalid input");

    (
        find_largest(&alu),
        find_smallest(&alu),
    )
}

enum Instruction {
    Inp(usize),
    Add(usize, Parameter),
    Mul(usize, Parameter),
    Div(usize, Parameter),
    Mod(usize, Parameter),
    Eq(usize, Parameter),
}

#[derive(Clone, Copy)]
enum Parameter {
    Register(usize),
    Literal(isize),
}

fn reg_idx(register: char) -> usize {
    match register {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => unreachable!()
    }
}

impl FromStr for Instruction {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        let line = line.trim();
        if line.starts_with("inp") {
            let (_, a) = line.split(' ').collect_tuple().unwrap();
            let a = reg_idx(a.chars().next().unwrap());
            return Ok(Instruction::Inp(a))
        }

        let instruction = match line.split(' ').collect_tuple() {
            Some((instr, a, b)) => {
                let a = reg_idx(a.chars().next().unwrap());
                let b = match b {
                    r @ ("w"|"x"|"y"|"z") => Parameter::Register(reg_idx(r.chars().next().unwrap())),
                    n => Parameter::Literal(n.parse().unwrap()),
                };
                match instr {
                    "add" => {Ok(Instruction::Add(a, b))}
                    "mul" => {Ok(Instruction::Mul(a, b))}
                    "div" => {Ok(Instruction::Div(a, b))}
                    "mod" => {Ok(Instruction::Mod(a, b))}
                    "eql" => {Ok(Instruction::Eq(a, b))}
                    _ => unreachable!()
                }
            }
            _ => Err(AdventError::UnexpectedElement {
                found: line.to_string(), expected: &["inp", "add", "mul", "div", "mod", "eql"]
            })
        }?;

        Ok(
            instruction
        )
    }
}

struct Alu {
    instructions: Vec<Instruction>,
}

impl FromStr for Alu {
    type Err = AdventError;

    fn from_str(lines: &str) -> Result<Self, AdventError> {
        let instructions = lines.trim().split('\n').map(|line|
            line.parse()
        )
        .collect::<Result<_, _>>()?;

        Ok(
            Alu {
                instructions,
            }
        )
    }
}

impl Alu {
    fn eval(&self, register: &mut [isize; 4], mut value: isize) -> bool {
        let mut idx = 0;
        let mut input = [0; DIGITS];
        for i in (0..DIGITS).rev() {
            input[i] = value % 10;
            if input[i] == 0 {
                return false
            }
            value /= 10
        }

        for i in &self.instructions {
            match *i {
                Instruction::Inp(r) => {
                    register[r] = input[idx];
                    idx += 1;
                },
                Instruction::Add(r, v) => {
                    register[r] += match v {
                        Parameter::Literal(l) => l,
                        Parameter::Register(t) => register[t],
                    };
                },
                Instruction::Mul(r, v) => {
                    register[r] *= match v {
                        Parameter::Literal(l) => l,
                        Parameter::Register(t) => register[t],
                    }
                },
                Instruction::Div(r, v) => {
                    register[r] /= match v {
                        Parameter::Literal(l) => l,
                        Parameter::Register(t) => register[t],
                    }
                },
                Instruction::Mod(r, v) => {
                    register[r] %= match v {
                        Parameter::Literal(l) => l,
                        Parameter::Register(t) => register[t],
                    }
                },
                Instruction::Eq(r, v) => {
                    let val = match v {
                        Parameter::Literal(l) => l,
                        Parameter::Register(t) => register[t],
                    };
                    register[r] = if register[r] == val {1} else {0}
                }
            }
        }
        true
    }
}

fn find_largest(alu: &Alu) -> isize {
    // let lower = 11111111111111;
    // lower bounds from previous runs:
    // let lower = 93576119596947;
    // let lower = 96787319596977;
    // let lower = 98943519596997;
    // let lower = 98998519596997;
    let lower = 98998519596997;
    (lower..99999999999999).into_par_iter().rev().find_map_first(|cur| {
        let mut out = [0; 4];
        if alu.eval(&mut out, cur) && out[3] == 0 {
            println!("{} -> {:?}", cur, out);
            return Some(cur)
        }
        None
    }).unwrap()
}

fn find_smallest(alu: &Alu) -> isize {
    // let lower = 99999999999999;
    // upper and lower bounds from previous runs:
    // let upper = 55554119596963;
    // let upper = 38898419151491;
    let upper = 31954519151421;
    // let lower = 14111111111111;
    // let lower = 15111111111111;
    // let lower = 21111111111111;
    let lower = 31111111111111;
    (lower..upper).into_par_iter().find_map_first(|cur| {
        let mut out = [0; 4];
        if alu.eval(&mut out, cur) && out[3] == 0 {
            println!("{} -> {:?}", cur, out);
            return Some(cur)
        }
        None
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            inp x
            mul x -1
        ";

        let alu: Alu = input.parse().expect("invalid input");
        let mut out = [0; 4];
        alu.eval(&mut out, 5);
        assert_eq!(out, [0, -5, 0, 0]);

        let input = r"
            inp z
            inp x
            mul z 3
            eql z x
        ";

        let alu: Alu = input.parse().expect("invalid input");
        let mut out = [0; 4];
        alu.eval(&mut out, 35);
        assert_eq!(out, [0, 5, 0, 0]);

        let mut out = [0; 4];
        alu.eval(&mut out, 39);
        assert_eq!(out, [0, 9, 0, 1]);

    }

}
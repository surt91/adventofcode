use std::{collections::HashMap, fs, str::FromStr, num::ParseIntError};

use itertools::Itertools;

use crate::utils::InvalidInput;

pub fn run() {
    let input = fs::read_to_string("data/day04a.dat").expect("input file does not exist");
    let (order, boards) = parse(&input).expect("invalid input");

    println!("{}", bingo_first(&order, boards.clone()));
    println!("{}", bingo_last(&order, boards));
}

#[derive(Clone)]
struct Board {
    width: usize,
    height: usize,
    positions: HashMap<isize, (usize, usize)>,
    marks: Vec<Vec<bool>>,
    finished: bool
}

impl Board {
    fn mark(&mut self, number: isize) {
        if let Some(&(x, y)) = self.positions.get(&number) {
            self.marks[x][y] = true;
        }
    }

    fn finished(&mut self) -> bool {
        self.finished = self.finished
        || (0..self.width).any(|x| self.test_column(x))
        || (0..self.height).any(|y| self.test_line(y));

        self.finished
    }

    fn test_line(&self, y: usize) -> bool {
        (0..self.width).map(|x| self.marks[x][y])
            .all(|b| b)
    }

    fn test_column(&self, x: usize) -> bool {
        (0..self.height).map(|y| self.marks[x][y])
            .all(|b| b)
    }

    fn score(&mut self) -> Option<isize> {
        if self.finished() {
            let score = self.positions.iter()
                .map(|(&i, &(x, y))| if self.marks[x][y] { 0 } else { i })
                .sum();
            Some(score)
        } else {
            None
        }
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, ParseIntError> {
        let lines: Vec<Vec<isize>> = input.trim().split('\n')
            .map(|line| line.split_whitespace()
                .map(|i| i.parse())
                .collect::<Result<_, _>>()
            ).collect::<Result<_, _>>()?;

        let width = lines[0].len();
        let height = lines.len();
        // assert that the board is not jagged
        assert!(lines.iter().all(|line| line.len() == width));

        let positions = (0..width).cartesian_product(0..height)
            .map(|(x, y)| (lines[x][y], (x, y)))
            .collect();

        let marks = vec![vec![false; height]; width];

        Ok(
            Board {
                width,
                height,
                positions,
                marks,
                finished: false
            }
        )
    }
}

fn bingo_first(order: &[isize], mut boards: Vec<Board>) -> isize {
    for &i in order {
        for board in &mut boards {
            board.mark(i);
            if let Some(score) = board.score() {
                return score * i;
            }
        }
    }

    panic!("There was no winner -> invalid input");
}

fn bingo_last(order: &[isize], mut boards: Vec<Board>) -> isize {
    let mut last_score = -1;
    for &i in order {
        for board in &mut boards {
            if !board.finished() {
                board.mark(i);
                if let Some(score) = board.score() {
                    last_score = score * i;
                }
            }
        }
    }

    last_score
}

fn parse(input: &str) -> Result<(Vec<isize>, Vec<Board>), InvalidInput> {
    let mut blocks = input.split("\n\n");

    let order = blocks.next()
        .ok_or(InvalidInput)?
        .split(',')
        .map(|s| s.parse().map_err(|_| InvalidInput))
        .collect::<Result<_, _>>()?;

    let mut boards = Vec::new();

    for block in blocks {
        if block.trim().is_empty() {
            return Err(InvalidInput);
        }
        let board = block.parse().map_err(|_| InvalidInput)?;
        boards.push(board);
    }

    Ok((order, boards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        ";

        let (order, boards) = parse(input).expect("invalid input");

        assert_eq!(bingo_first(&order, boards.clone()), 4512);
        assert_eq!(bingo_last(&order, boards), 1924);
    }
}

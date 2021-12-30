use std::{fs, str::FromStr, fmt};


use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day25a.dat").expect("input file does not exist");
    let mut region: Region = input.parse().expect("invalid input");

    (
        region.count(),
        0
    )
}

#[derive(PartialEq, Eq)]
enum Cucumber {
    East,
    South,
    None
}

struct Region {
    width: usize,
    height: usize,
    cucumbers: Vec<Vec<Cucumber>>
}

impl Region {
    fn move_east(&mut self) -> bool {
        let mut will_move: Vec<(usize, usize, usize)> = Vec::new();
        for j in 0..self.height {
            for i in 0..self.width {
                if let Cucumber::East = self.cucumbers[j][i] {
                    let next = if i + 1 == self.width { 0 } else { i + 1 };
                    if self.cucumbers[j][next] == Cucumber::None {
                        will_move.push((i, j, next));
                    }
                }
            }
        }

        for (i, j, next) in &will_move {
            self.cucumbers[*j][*next] = Cucumber::East;
            self.cucumbers[*j][*i] = Cucumber::None;
        }

        !will_move.is_empty()
    }

    fn move_south(&mut self) -> bool {
        let mut will_move: Vec<(usize, usize, usize)> = Vec::new();
        for j in 0..self.height {
            for i in 0..self.width {
                if let Cucumber::South = self.cucumbers[j][i] {
                    let next = if j + 1 == self.height { 0 } else { j + 1 };
                    if self.cucumbers[next][i] == Cucumber::None {
                        will_move.push((i, j, next));
                    }
                }
            }
        }

        for (i, j, next) in &will_move {
            self.cucumbers[*next][*i] = Cucumber::South;
            self.cucumbers[*j][*i] = Cucumber::None;
        }

        !will_move.is_empty()
    }

    fn count(&mut self) -> usize {
        let mut ctr = 0;
        loop {
            ctr += 1;
            let mut moved = false;
            moved |= self.move_east();
            moved |= self.move_south();

            if !moved {
                break ctr
            }
        }
    }
}

impl FromStr for Region {
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let cucumbers: Vec<Vec<Cucumber>> = input.trim().split('\n')
            .map(|line| line.trim().chars()
                .map(|i| match i {
                    '>' => Ok(Cucumber::East),
                    'v' => Ok(Cucumber::South),
                    '.' => Ok(Cucumber::None),
                    val => Err(AdventError::UnexpectedElement { found: val.to_string(), expected: &[">", "v", "."] })
                })
                .collect::<Result<_, _>>()
            ).collect::<Result<_, _>>()?;

        let width = cucumbers[0].len();
        let height = cucumbers.len();
        // assert that the board is not jagged
        assert!(cucumbers.iter().all(|line| line.len() == width));

        Ok(
            Region {
                width,
                height,
                cucumbers,
            }
        )
    }
}

impl fmt::Display for Region
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{} ", self.cucumbers[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Cucumber
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cucumber::East => write!(f, ">"),
            Cucumber::South => write!(f, "v"),
            Cucumber::None => write!(f, ".")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        ";

        let mut region: Region = input.parse().expect("invalid input");

        assert_eq!(region.count(), 58);
    }
}



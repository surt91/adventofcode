use core::fmt;
use std::{str::FromStr, iter, ops::{Index, IndexMut}};

use super::AdventError;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub values: Vec<Vec<u8>>
}

impl Map {
    pub fn neighbors(&self, coordinate: (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
        let (x, y) = coordinate;
        iter::once(
            if y == 0 {None} else {Some((x, y-1))}
        ).chain(iter::once(
            if y >= self.height - 1  {None} else {Some((x, y+1))},
        )).chain(iter::once(
            if x == 0 {None} else {Some((x-1, y))},
        )).chain(iter::once(
            if x >= self.width - 1 {None} else {Some((x+1, y))},
        )).flatten()
    }

    pub fn diagonal_neighbors(&self, coordinate: (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
        let (x, y) = coordinate;
        self.neighbors(coordinate).map(Some).chain(iter::once(
            if x >= self.width - 1 || y >= self.height - 1 {None} else {Some((x+1, y+1))},
        )).chain(iter::once(
            if x ==0 || y >= self.height - 1 {None} else {Some((x-1, y+1))},
        )).chain(iter::once(
            if x >= self.width - 1 || y == 0 {None} else {Some((x+1, y-1))},
        )).chain(iter::once(
            if x == 0 || y == 0 {None} else {Some((x-1, y-1))},
        )).flatten()
    }
}

impl Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, coordinate: (usize, usize)) -> &Self::Output {
        let (x, y) = coordinate;
        &self.values[y][x]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, coordinate: (usize, usize)) -> &mut Self::Output {
        let (x, y) = coordinate;
        &mut self.values[y][x]
    }
}

impl fmt::Display for Map
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{} ", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Map
{
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let values: Vec<Vec<u8>> = input.trim().split('\n')
            .map(|line| line.trim().chars()
                .map(|c|
                    c.to_digit(10)
                    .map(|x| x as u8)
                    .ok_or(
                        AdventError::UnexpectedElement{found: c.to_string(), expected: vec!["a number".to_string()]})
                    ).collect::<Result<_, _>>()
                ).collect::<Result<_, _>>()?;

        let width = values[0].len();
        let height = values.len();

        assert!(values.iter().all(|l| l.len() == width));

        Ok(
            Map {
                width,
                height,
                values
            }
        )
    }
}
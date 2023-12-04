use core::fmt;
use std::{str::FromStr, iter, ops::{Index, IndexMut}, fmt::Display};

use itertools::Itertools;

use super::{AdventError, shortest_path::Neighborful};

pub type Coord = (usize, usize);

pub struct Map<T> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<Vec<T>>
}

impl<T> Map<T> {
    pub fn diagonal_neighbors(&self, coordinate: Coord) -> impl Iterator<Item=Coord> {
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
        .collect_vec()
        .into_iter()
    }
}

impl<T> Index<&Coord> for Map<T> {
    type Output = T;

    fn index(&self, coordinate: &Coord) -> &Self::Output {
        let (x, y) = coordinate;
        &self.values[*y][*x]
    }
}

impl<T> Index<Coord> for &Map<T> {
    type Output = T;

    fn index(&self, coordinate: Coord) -> &Self::Output {
        let (x, y) = coordinate;
        &self.values[y][x]
    }
}

impl<T> Index<Coord> for Map<T> {
    type Output = T;

    fn index(&self, coordinate: Coord) -> &Self::Output {
        let (x, y) = coordinate;
        &self.values[y][x]
    }
}

impl<T> IndexMut<Coord> for Map<T> {
    fn index_mut(&mut self, coordinate: Coord) -> &mut Self::Output {
        let (x, y) = coordinate;
        &mut self.values[y][x]
    }
}

impl<T: Display> fmt::Display for Map<T>
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

impl FromStr for Map<u8>
{
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let values: Vec<Vec<u8>> = input.trim().split('\n')
            .map(|line| line.trim().chars()
                .map(|c|
                    c.to_digit(10)
                    .map(|x| x as u8)
                    .ok_or(
                        AdventError::UnexpectedElement{
                            found: c.to_string(),
                            expected: &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
                        })
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


impl FromStr for Map<char>
{
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let values: Vec<Vec<char>> = input.trim().split('\n')
            .map(|line| line.trim().chars().collect())
            .collect();

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

impl<T> Neighborful<Coord> for &Map<T> {
    fn neighbors(&self, coordinate: Coord) -> impl Iterator<Item=Coord> {
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

    fn distance(c1: Coord, c2: Coord) -> usize {
        // assume all costs are >= 1 => h is manhattan distance:
        ((c1.0 as isize - c2.0 as isize).abs() + (c1.1 as isize - c2.1 as isize).abs()) as usize
    }
}

use std::{collections::HashMap, str::FromStr, fmt, cmp::max};

use crate::utils::{AdventError, read_lines};

pub fn run() -> (usize, usize) {
    let lines = read_lines("data/day05a.dat");
    let data = parse(&lines).expect("invalid input");

    (
        count(&data, 2, false),
        count(&data, 2, true)
    )
}

fn count(data: &[(Point, Point)], threshold: usize, diagonals: bool) -> usize {
    let mut map: HashMap<Point, usize> = HashMap::new();

    for (start, end) in data {
        let (dx, dy) = ((end.x - start.x).abs(), (end.y - start.y).abs());

        // skip diagonals
        if !diagonals && dx == dy {
            continue;
        }

        let mut x = start.x;
        let mut y = start.y;
        for _ in 0..=max(dx, dy) {
            let p = Point::new(x, y);

            *map.entry(p).or_insert(0) += 1;

            x += (end.x - start.x).signum();
            y += (end.y - start.y).signum();
        }
    }

    map.values()
        .filter(|&&i| i >= threshold)
        .count()
}

fn _print_map(map: &HashMap<Point, usize>) {
    for j in 0..10 {
        for i in 0..10 {
            let c = map.get(&Point::new(i, j)).or(Some(&0)).unwrap();
            print!("{} ", c);
        }
        println!();
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point {x, y}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let mut p = input.split(',');

        let x = p.next().ok_or(AdventError::NotEnoughElements)?
            .parse()?;
        let y = p.next().ok_or(AdventError::NotEnoughElements)?
            .parse()?;

        Ok(Point {x, y})
    }
}

fn parse(lines: &[String]) -> Result<Vec<(Point, Point)>, AdventError> {
    fn parse_line(line: &str) -> Result<(Point, Point), AdventError> {
        let mut points = line.split(" -> ");

        let p1 = points.next()
            .ok_or(AdventError::NotEnoughElements)?
            .parse()?;
        let p2 = points.next()
            .ok_or(AdventError::NotEnoughElements)?
            .parse()?;

        Ok((p1, p2))
    }

    lines.iter()
        .map(|line|
            parse_line(line)
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example() {
        let input = r"
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        ";

        let lines = split_lines(input);
        let data = parse(&lines).expect("invalid input");

        assert_eq!(count(&data, 2, false), 5);
        assert_eq!(count(&data, 2, true), 12);
    }
}
use std::{str::FromStr, collections::HashSet};

use itertools::Itertools;

use crate::{utils::AdventError, data_str};

pub fn run() -> (usize, String) {
    let input = data_str!("day13a");
    let (points, folds) = parse(input).expect("invalid input");

    (
        fold_once(&points, &folds[0]),
        print_code(&points, &folds)
    )
}

enum Fold {
    X(usize),
    Y(usize)
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize
}

impl FromStr for Fold {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        let assignment = line.split(' ').last().ok_or(AdventError::NotEnoughElements)?;
        let mut it = assignment.split('=');
        let axis = it.next().ok_or(AdventError::NotEnoughElements)?;
        let position: usize = it.next().ok_or(AdventError::NotEnoughElements)?.parse()?;
        match axis {
            "x" => Ok(Fold::X(position)),
            "y" => Ok(Fold::Y(position)),
            s => Err(AdventError::UnexpectedElement { found: s.to_string(), expected: &["x", "y"] })
        }
    }
}

impl FromStr for Point {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        let mut it = line.trim().split(',');
        let x: usize = it.next().ok_or(AdventError::NotEnoughElements)?.parse()?;
        let y: usize = it.next().ok_or(AdventError::NotEnoughElements)?.parse()?;
        Ok(Point {
            x,
            y
        })
    }
}

fn parse(input: &str) -> Result<(Vec<Point>, Vec<Fold>), AdventError> {
    let mut blocks = input.trim().split("\n\n");

    let points = blocks.next()
        .ok_or(AdventError::NotEnoughElements)?
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let folds = blocks.next()
        .ok_or(AdventError::NotEnoughElements)?
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok((points, folds))
}

fn fold_once(points: &[Point], f: &Fold) -> usize {
    let mut points = points.iter().cloned().collect();
    points = fold(&points, f);

    points.len()
}

fn print_code(points: &[Point], folds: &[Fold]) -> String {
    let mut points = points.iter().cloned().collect();
    for f in folds {
        points = fold(&points, f);
    }

    let paper = draw_points_on_paper_transposed(&points);

    let sol = paper.split("\n\n")
        .map(recognize_letter)
        .collect::<Result<_,_>>();
    match sol {
        Ok(solution) => solution,
        Err(_) => {
            println!("{}", draw_points_on_paper(&points));
            "# parsing failed".to_string()}
    }
}

fn fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    points.iter().map(|&Point{x, y}| {
        let (x, y) = match fold {
            Fold::Y(y_fold) => {
                assert!(y != *y_fold);
                if y > *y_fold {
                    (x, 2*y_fold-y)
                } else {
                    (x, y)
                }
            },
            Fold::X(x_fold) => {
                assert!(x != *x_fold);
                if x > *x_fold {
                    (2*x_fold-x, y)
                } else {
                    (x, y)
                }
            }
        };
        Point {x, y}
    }).collect()
}

fn draw_points_on_paper_transposed(points: &HashSet<Point>) -> String {
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    let mut paper = vec![vec![false; y_max+1]; x_max+1];
    for p in points {
        paper[p.x][p.y] = true;
    }

    paper_to_string(&paper)
}

fn draw_points_on_paper(points: &HashSet<Point>) -> String {
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    let mut paper = vec![vec![false; x_max+1]; y_max+1];
    for p in points {
        paper[p.y][p.x] = true;
    }

    paper_to_string(&paper)
}

fn paper_to_string(paper: &[Vec<bool>]) -> String {
    paper.iter().map(|row| {
        row.iter().map(|&p| {
            if p {"#"} else {" "}
        }).join("")
    }).map(|line| line.trim_end().to_string())
    .join("\n")
}

fn recognize_letter(input: &str) -> Result<String, AdventError> {
    use crate::utils::letters::parse;

    parse(input.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5
        ";

        let (points, folds) = parse(input).expect("invalid input");

        assert_eq!(fold_once(&points, &folds[0]), 17);
        assert_eq!(print_code(&points, &folds), "O".to_string());
    }
}

use std::{fs, str::FromStr};

use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day13a.dat").expect("input file does not exist");
    let (points, folds) = parse(&input).expect("invalid input");

    (
        fold_once(&points, &folds[0]),
        print_code(&points, &folds)
    )
}

enum Fold {
    X(usize),
    Y(usize)
}

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
            s => Err(AdventError::UnexpectedElement { found: s.to_string(), expected: vec!["x".to_string(), "y".to_string()] })
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
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    let mut paper = vec![vec![false; y_max+1]; x_max+1];
    for p in points {
        paper[p.x][p.y] = true;
    }

    paper = fold(&paper, f);

    paper.iter().flatten().filter(|&&x| x).count()
}

fn print_code(points: &[Point], folds: &[Fold]) -> usize {
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    let mut paper = vec![vec![false; y_max+1]; x_max+1];
    for p in points {
        paper[p.x][p.y] = true;
    }

    for f in folds {
        paper = fold(&paper, f);
    }

    print_paper(&paper);

    paper.iter().flatten().filter(|&&x| x).count()
}

fn fold(paper: &[Vec<bool>], fold: &Fold) -> Vec<Vec<bool>> {
    let x_max = paper.len() - 1;
    let y_max = paper[0].len() - 1;

    match fold {
        Fold::Y(y_fold) => {
            let mut paper_out = vec![vec![false; *y_fold]; x_max+1];
            for x in 0..=x_max {
                assert!(!paper[x][*y_fold]);
                for y in 0..*y_fold {
                    paper_out[x][y] = paper[x][y] | paper[x][y_max-y];
                }
            }
            paper_out
        },
        Fold::X(x_fold) => {
            let mut paper_out = vec![vec![false; y_max+1]; *x_fold];
            for y in 0..=y_max {
                assert!(!paper[*x_fold][y]);
                for x in 0..*x_fold {
                    paper_out[x][y] = paper[x][y] | paper[x_max-x][y];
                }
            }
            paper_out
        }
    }
}

fn print_paper(paper: &[Vec<bool>]) {
    for row in paper.iter() {
        for &p in row {
            print!("{}", if p {"#"} else {" "});
        }
        println!()
    }
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
        assert_eq!(print_code(&points, &folds), 16);
    }
}

use crate::utils::{AdventError, read_lines};

pub fn run() -> (isize, isize) {
    let lines = read_lines("data/day07a.dat");
    let data = parse(&lines[0]).expect("invalid input");

    (
        fuel_cost(&data, false),
        fuel_cost(&data, true),
    )
}

fn fuel_cost(positions: &[isize], correct: bool) -> isize {
    // I could use a far better minimization here, but for the small input something stupid is good enough
    let &low = positions.iter().min().unwrap();
    let &high = positions.iter().max().unwrap();

    (low..=high).map(|x| cost(positions, x, correct))
        .min()
        .unwrap()
}

fn cost(positions: &[isize], test: isize, correct: bool) -> isize {
    let tmp = positions.iter()
        .map(|x| (x - test).abs());

    if correct {
        tmp.map(|dist| (dist*(dist+1)) / 2)
            .sum()
    } else {
        tmp.sum()
    }
}

fn parse(input: &str) -> Result<Vec<isize>, AdventError> {
    input.split(',')
        .map(|i|
            i.parse()
                .map_err(AdventError::Parser)
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"16,1,2,0,4,2,7,1,2,14";

        let data = parse(input).expect("invalid input");

        assert_eq!(fuel_cost(&data, false), 37);
        assert_eq!(fuel_cost(&data, true), 168);
    }
}
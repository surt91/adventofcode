use crate::utils::{AdventError, read_lines};

pub fn run() -> (isize, isize) {
    let lines = read_lines("data/day07a.dat");
    let data = parse(&lines[0]).expect("invalid input");

    (
        fuel_cost(&data),
        0
    )
}

fn fuel_cost(positions: &[isize]) -> isize {
    // I could use a far better minimization here, but for the small input something stupid is good enough
    let &low = positions.iter().min().unwrap();
    let &high = positions.iter().max().unwrap();

    let mut best_cost = (high - low) * positions.len() as isize;

    for i in low..=high {
        let c = cost(positions, i);
        if c < best_cost {
            best_cost = c;
        }
    }

    best_cost
}

fn cost(positions: &[isize], test: isize) -> isize {
    positions.iter().map(|x| (x - test).abs()).sum()
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

        assert_eq!(fuel_cost(&data), 37);
    }
}
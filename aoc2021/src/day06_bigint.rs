use num_bigint::BigUint;

use crate::utils::{AdventError, read_lines};

pub fn run() -> (usize, usize) {
    let lines = read_lines("data/day06a.dat");
    let data = parse(&lines[0]).expect("invalid input");

    let tmp = num_fish(&data, 400000);
    println!("{}", tmp);
    println!("~10^{}", format!("{}", tmp).len()-1);

    (
        // num_fish(&data, 80),
        // num_fish(&data, 256),
        0,0
    )
}

fn num_fish(initial_conditions: &[u8], iterations: usize) -> BigUint {
    let mut ls = LanternfishSchool::new(initial_conditions);
    for _ in 0..iterations {
        ls.step();
    }
    ls.count()
}

struct LanternfishSchool {
    // at each index is the number of fish with the `internal timer` equal to the index
    internal_timers: Vec<BigUint>
}

impl LanternfishSchool {
    fn new(individuals: &[u8]) -> Self {
        let mut internal_timers = vec![BigUint::new(vec![0]); 9];

        for &i in individuals {
            internal_timers[i as usize] += BigUint::new(vec![1]);
        }

        LanternfishSchool {
            internal_timers
        }
    }

    fn count(&self) -> BigUint {
        self.internal_timers
            .iter()
            .sum()
    }

    fn step(&mut self) {
        let new_fish = self.internal_timers.remove(0);
        self.internal_timers.push(new_fish.clone());
        self.internal_timers[6] += new_fish;
    }
}

fn parse(input: &str) -> Result<Vec<u8>, AdventError> {
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
        let input = r"3,4,3,1,2";

        let data = parse(input).expect("invalid input");

        assert_eq!(num_fish(&data, 18), 26);
        assert_eq!(num_fish(&data, 80), 5934);
        assert_eq!(num_fish(&data, 256), 26984457539);
    }
}
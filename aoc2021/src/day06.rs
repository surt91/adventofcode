use std::collections::VecDeque;

use crate::utils::{read_lines, parse_single_line};

pub fn run() -> (usize, usize) {
    let lines = read_lines("data/day06a.dat");
    let data = parse_single_line(&lines[0]).expect("invalid input");

    (
        num_fish(&data, 80),
        num_fish(&data, 256),
    )
}

fn num_fish(initial_conditions: &[u8], iterations: usize) -> usize {
    let mut ls = LanternfishSchool::new(initial_conditions);
    for _ in 0..iterations {
        ls.step();
    }
    ls.count()
}

struct LanternfishSchool {
    // at each index is the number of fish with the `internal timer` equal to the index
    internal_timers: VecDeque<usize>
}

impl LanternfishSchool {
    fn new(individuals: &[u8]) -> Self {
        let mut internal_timers = VecDeque::from(vec![0; 9]);

        for &i in individuals {
            internal_timers[i as usize] += 1;
        }

        LanternfishSchool {
            internal_timers
        }
    }

    fn count(&self) -> usize {
        self.internal_timers
            .iter()
            .sum()
    }

    fn step(&mut self) {
        self.internal_timers.rotate_left(1);
        self.internal_timers[6] += self.internal_timers[8];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"3,4,3,1,2";

        let data = parse_single_line(input).expect("invalid input");

        assert_eq!(num_fish(&data, 18), 26);
        assert_eq!(num_fish(&data, 80), 5934);
        assert_eq!(num_fish(&data, 256), 26984457539);
    }
}
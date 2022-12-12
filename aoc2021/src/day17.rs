use std::{str::FromStr, cmp};

use itertools::Itertools;
use scan_fmt::scan_fmt;

use crate::{utils::AdventError, data_str};

pub fn run() -> (isize, usize) {
    let line = data_str!("day17a");
    let target: TargetZone = line.parse().expect("invalid input");

    (
        target.highest_point(),
        target.count_trajectories(),
    )
}

struct TargetZone {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl TargetZone {
    fn highest_point(&self) -> isize {
        let vx_max: isize = self.x_max;
        let vy_min: isize = 0;
        let vy_max: isize = -self.y_min;

        (0..vx_max).cartesian_product(vy_min..vy_max)
            .filter_map(|(vx, vy)| self.test(vx, vy))
            .max()
            .unwrap()
    }

    fn count_trajectories(&self) -> usize {
        let vx_max: isize = self.x_max;
        let vy_min: isize = self.y_min;
        let vy_max: isize = cmp::max(self.y_max, self.y_min.abs());

        (0..=vx_max).cartesian_product(vy_min..=vy_max)
            .filter_map(|(vx, vy)| self.test(vx, vy))
            .count()
    }

    fn test(&self, mut vx: isize, mut vy: isize) -> Option<isize> {
        let mut x = 0;
        let mut y = 0;

        let mut high = None;
        let mut watermark = 0;

        while x <= self.x_max && y >= self.y_min {
            x += vx;
            if vx > 0 {
                vx -= 1;
            }

            y += vy;
            vy -= 1;

            watermark = cmp::max(y, watermark);

            if self.contains(x, y) {
                high = Some(watermark);
            }
        }

        high
    }

    fn contains(&self, x: isize, y: isize) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

impl FromStr for TargetZone {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        let (x_min, x_max, y_min, y_max) = scan_fmt!(
            line,
            "target area: x={}..{}, y={}..{}",
            isize, isize, isize, isize
        )?;

        Ok(TargetZone {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"target area: x=20..30, y=-10..-5";

        let target: TargetZone = input.parse().expect("invalid input");

        assert_eq!(target.test(6, 0), Some(0));
        assert_eq!(target.highest_point(), 45);
        assert_eq!(target.count_trajectories(), 112);
    }
}
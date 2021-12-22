use std::{fs, str::FromStr, cmp, collections::HashSet};

use scan_fmt::scan_fmt;

use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day22a.dat").expect("input file does not exist");
    let cuboids: Cuboids = input.parse().expect("invalid input");

    (
        cuboids.count_on(),
        0,
    )
}

#[derive(Hash, PartialEq, Eq)]
struct Cuboid {
    on: bool,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl FromStr for Cuboid {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        let (
            on,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        ) = scan_fmt!(
            line.trim(),
            "{} x={}..{},y={}..{},z={}..{}",
            String, isize, isize, isize, isize, isize, isize
        )?;

        let on = on == "on";

        Ok(
            Cuboid {
                on,
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            }
        )
    }
}

struct Cuboids {
    members: Vec<Cuboid>
}

impl FromStr for Cuboids {
    type Err = AdventError;

    fn from_str(lines: &str) -> Result<Self, AdventError> {
        let members = lines.trim().split('\n').map(|line|
            line.parse()
        )
        .collect::<Result<_, _>>()?;

        Ok(
            Cuboids {
                members
            }
        )
    }
}

impl Cuboids {
    fn count_on(&self) -> usize {
        let mut singles = HashSet::new();

        for c in &self.members {
            let x_min = cmp::max(c.x_min, -50);
            let x_max = cmp::min(c.x_max, 50);
            for x in x_min..=x_max {
                let y_min = cmp::max(c.y_min, -50);
                let y_max = cmp::min(c.y_max, 50);
                for y in y_min..=y_max {
                    let z_min = cmp::max(c.z_min, -50);
                    let z_max = cmp::min(c.z_max, 50);
                    for z in z_min..=z_max {
                        if c.on {
                            singles.insert((x, y, z));
                        } else {
                            singles.remove(&(x, y, z));
                        }
                    }
                }
            }
        }

        singles.len()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            on x=-20..26,y=-36..17,z=-47..7
            on x=-20..33,y=-21..23,z=-26..28
            on x=-22..28,y=-29..23,z=-38..16
            on x=-46..7,y=-6..46,z=-50..-1
            on x=-49..1,y=-3..46,z=-24..28
            on x=2..47,y=-22..22,z=-23..27
            on x=-27..23,y=-28..26,z=-21..29
            on x=-39..5,y=-6..47,z=-3..44
            on x=-30..21,y=-8..43,z=-13..34
            on x=-22..26,y=-27..20,z=-29..19
            off x=-48..-32,y=26..41,z=-47..-37
            on x=-12..35,y=6..50,z=-50..-2
            off x=-48..-32,y=-32..-16,z=-15..-5
            on x=-18..26,y=-33..15,z=-7..46
            off x=-40..-22,y=-38..-28,z=23..41
            on x=-16..35,y=-41..10,z=-47..6
            off x=-32..-23,y=11..30,z=-14..3
            on x=-49..-5,y=-3..45,z=-29..18
            off x=18..30,y=-20..-8,z=-3..13
            on x=-41..9,y=-7..43,z=-33..15
            on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
            on x=967..23432,y=45373..81175,z=27513..53682
        ";

        let cuboids: Cuboids = input.parse().expect("invalid input");

        assert_eq!(cuboids.count_on(), 590784);
    }
}
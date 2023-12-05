use std::str::FromStr;

use scan_fmt::scan_fmt;

use aoc2021::data_str;
use aoc2021::utils::AdventError;

struct Line {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

struct Map {
    lines: Vec<Line>,
}

impl Map {
    // assume there are no overlaps
    fn map(&self, src: u64) -> u64 {
        for line in &self.lines {
            if (line.src_start..line.src_start+line.length).contains(&src) {
                let offset = src - line.src_start;
                return line.dest_start + offset
            }
        }
        src
    }
}

struct Maps {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl FromStr for Maps {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = Vec::new();
        let mut blocks = s.split("\n\n");
        let seeds = blocks.next().ok_or(AdventError::NotEnoughElements)?;
        let seeds: Vec<u64> = seeds.trim()
            .strip_prefix("seeds:")
            .ok_or(AdventError::UnexpectedElement { found: seeds.to_string(), expected: &["seeds: *"] })?
            .trim()
            .split(' ')
            .map(|s| s.parse::<u64>())
            .collect::<Result<_, _>>()?;

        for block in blocks {
            let mut lines = Vec::new();
            for line in block.trim().split('\n').skip(1) {
                let (dest_start, src_start, length) = scan_fmt!(
                    line,
                    "{} {} {}",
                    u64, u64, u64
                )?;
                lines.push(
                    Line {
                        dest_start,
                        src_start,
                        length,
                    }
                )
            }
            maps.push(Map { lines });
        }
        Ok(Maps{
            seeds,
            maps,
        })
    }
}

impl Maps {
    fn map(&self, mut src: u64) -> u64 {
        for map in &self.maps {
            src = map.map(src)
        }
        src
    }
}

pub fn run() -> (u64, u64) {

    let input = data_str!("day05");
    let maps: Maps = input.parse().expect("invalid input");

    (
        lowest_location_number(&maps),
        0,
    )
}

fn lowest_location_number(maps: &Maps) -> u64 {
    maps.seeds.iter().map(|&s| maps.map(s)).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        ";

        let maps: Maps = input.parse().expect("invalid input");

        assert_eq!(lowest_location_number(&maps), 35);
    }
}
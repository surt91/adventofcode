use std::collections::HashMap;
use std::str::FromStr;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

const LIMIT: Cubes = Cubes {red: 12, green: 13, blue: 14};

impl Cubes {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Cubes {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in s.split(',') {
            let (num, color) = cube.trim()
                .split_once(' ')
                .ok_or(AdventError::NotEnoughElements)?;
            let num: u32 = num.parse()?;

            match color {
                "red" => cubes.red = num,
                "green" => cubes.green = num,
                "blue" => cubes.blue = num,
                _ => Err(AdventError::UnexpectedElement {
                    found: color.to_string(), expected: &["red", "green", "blue"]
                })?
            }
        }

        Ok(cubes)
    }
}

pub fn run() -> (u32, u32) {

    let input = data_str!("day02");
    let data = parse(input).expect("invalid input");

    (
        sum_of_valid_ids(&data, LIMIT),
        power_sum_of_minimum_cube_set(&data),
    )
}

fn parse(input: &str) -> Result<HashMap<u32, Vec<Cubes>>, AdventError> {
    split_lines(input).iter().map(|game| {
        let (id, rest) = game.split_once(':').ok_or(AdventError::NotEnoughElements)?;
        let id = id.strip_prefix("Game ")
            .ok_or(AdventError::UnexpectedElement { found: id.to_string(), expected: &["Game "] })?;
        let id: u32 = id.parse()?;
        let cubes: Vec<Cubes> = rest.trim()
            .split(';')
            .map(|set| set.trim().parse::<Cubes>())
            .collect::<Result<_, AdventError>>()?;
        Ok((id, cubes))
    })
    .collect()
}

fn sum_of_valid_ids(results: &HashMap<u32, Vec<Cubes>>, limit: Cubes) -> u32 {
    results.iter()
        .filter(|(_id, games)| {
            games.iter()
                .all(|cubes| cubes.red <= limit.red && cubes.green <= limit.green && cubes.blue <= limit.blue)
        })
        .map(|(id, _games)| id)
        .sum()
}

fn minimum_cube_set(game: &[Cubes]) -> Cubes {
    let red = game.iter().map(|cubes| cubes.red).max().unwrap_or(0);
    let green = game.iter().map(|cubes| cubes.green).max().unwrap_or(0);
    let blue = game.iter().map(|cubes| cubes.blue).max().unwrap_or(0);

    Cubes {
        red,
        green,
        blue,
    }
}

fn power_sum_of_minimum_cube_set(results: &HashMap<u32, Vec<Cubes>>) -> u32 {
    results.iter().map(|(_id, game)| minimum_cube_set(game))
        .map(|cube_set| cube_set.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let data = parse(input).expect("invalid input");

        assert_eq!(sum_of_valid_ids(&data, LIMIT), 8);
        assert_eq!(power_sum_of_minimum_cube_set(&data), 2286);
    }
}
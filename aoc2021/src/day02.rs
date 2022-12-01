use crate::{utils::{AdventError, split_lines}, data_str};

pub fn run() -> (i64, i64) {
    let input = data_str!("day02a");
    let lines = split_lines(input);
    let actions = parse(&lines).expect("invalid input");

    (
        navigate(&actions),
        aim(&actions)
    )
}

fn navigate(actions: &[Direction]) -> i64 {
    let mut depth = 0i64;
    let mut position = 0i64;

    for action in actions {
        match action {
            Direction::Forward(magnitude) => position += magnitude,
            Direction::Up(magnitude) => depth -= magnitude,
            Direction::Down(magnitude) => depth += magnitude,
        };
    }

    depth * position
}

fn aim(actions: &[Direction]) -> i64 {
    let mut aim = 0i64;
    let mut depth = 0i64;
    let mut position = 0i64;

    for action in actions {
        match action {
            Direction::Forward(magnitude) => {
                position += magnitude;
                depth += aim * magnitude;
            },
            Direction::Up(magnitude) => aim -= magnitude,
            Direction::Down(magnitude) => aim += magnitude,
        };
    }

    depth * position
}

enum Direction {
    Forward(i64),
    Up(i64),
    Down(i64)
}

fn parse(input: &[String]) -> Result<Vec<Direction>, AdventError> {
    input.iter().map(|action| {
        if let [direction, magnitude_string] = action.split(' ').collect::<Vec<&str>>()[..] {
            let magnitude = magnitude_string.parse()?;
            match direction {
                "forward" => Ok(Direction::Forward(magnitude)),
                "up" => Ok(Direction::Up(magnitude)),
                "down" => Ok(Direction::Down(magnitude)),
                _ => Err(AdventError::UnexpectedElement {
                    found: direction.to_string(),
                    expected: &["forward", "up", "down"]
                })
            }
        } else {
            Err(AdventError::NotEnoughElements)
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example() {
        let input = r"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        ";

        let lines = split_lines(input);
        let actions = parse(&lines).expect("invalid input");

        assert_eq!(navigate(&actions), 150);
        assert_eq!(aim(&actions), 900);
    }
}
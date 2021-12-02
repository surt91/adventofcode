use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/day02a.dat").unwrap();
    println!("{}", navigate(&input));
    println!("{}", aim(&input));
}

fn navigate(input: &str) -> i64 {
    let actions = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let mut depth = 0i64;
    let mut position = 0i64;
    for action in actions {

        if let [direction, magnitude_string] = action.split(" ").collect::<Vec<&str>>()[..] {
            let magnitude = magnitude_string.parse::<i64>().expect("invalid input");
            match direction {
                "forward" => position += magnitude,
                "up" => depth -= magnitude,
                "down" => depth += magnitude,
                _ => panic!("invalid input")
            }
        }
    }

    return depth * position;
}

fn aim(input: &str) -> i64 {
    let actions = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let mut aim = 0i64;
    let mut depth = 0i64;
    let mut position = 0i64;
    for action in actions {
        if let [direction, magnitude_string] = action.split(" ").collect::<Vec<&str>>()[..] {
            let magnitude = magnitude_string.parse::<i64>().expect("invalid input");
            match direction {
                "forward" => {
                    position += magnitude;
                    depth += aim * magnitude;
                },
                "up" => aim -= magnitude,
                "down" => aim += magnitude,
                _ => panic!("invalid input")
            }
        }
    }

    return depth * position;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "#;
        assert_eq!(navigate(input), 150);
        assert_eq!(aim(input), 900);
    }
}
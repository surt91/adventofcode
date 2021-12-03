use super::utils::read_lines;

pub fn run() {
    let actions = read_lines("data/day02a.dat");

    println!("{}", navigate(&actions));
    println!("{}", aim(&actions));
}

fn navigate(actions: &[String]) -> i64 {
    let mut depth = 0i64;
    let mut position = 0i64;
    for action in actions {
        if let [direction, magnitude_string] = action.split(' ').collect::<Vec<&str>>()[..] {
            let magnitude = magnitude_string.parse::<i64>().expect("invalid input");
            match direction {
                "forward" => position += magnitude,
                "up" => depth -= magnitude,
                "down" => depth += magnitude,
                _ => panic!("invalid input")
            }
        }
    }

    depth * position
}

fn aim(actions: &[String]) -> i64 {
    let mut aim = 0i64;
    let mut depth = 0i64;
    let mut position = 0i64;
    for action in actions {
        if let [direction, magnitude_string] = action.split(' ').collect::<Vec<&str>>()[..] {
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
        } else {
            panic!("invalid input")
        }
    }

    depth * position
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example1() {
        let input = r"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        ";

        let actions = split_lines(input);

        assert_eq!(navigate(&actions), 150);
        assert_eq!(aim(&actions), 900);
    }
}
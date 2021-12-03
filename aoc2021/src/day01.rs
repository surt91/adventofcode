use itermore::IterMore;

use super::utils::read_lines;

pub fn run() {
    let lines = read_lines("data/day01a.dat");
    let data = parse(&lines);

    println!("{}", sonar(&data));
    println!("{}", three(&data));
}

fn sonar(data: &[i32]) -> i32 {
    data.iter()
        .windows()
        .filter(|&[a, b]| a < b)
        .count() as i32
}

fn three(data: &[i32]) -> i32 {
    let first = data.iter().windows();
    let second = data.iter().skip(1).windows();

    first.zip(second)
        .filter(|&([a1, a2, a3], [b1, b2, b3])| a1+a2+a3 < b1+b2+b3)
        .count() as i32
}

fn parse(lines: &[String]) -> Vec<i32> {
    lines.iter()
        .map(|line|
            line.parse::<i32>()
                .expect("invalid input data!")
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example1() {
        let input = r"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        ";

        let lines = split_lines(input);
        let data = parse(&lines);

        assert_eq!(sonar(&data), 7);
        assert_eq!(three(&data), 5);
    }
}
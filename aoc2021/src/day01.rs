use std::fs;
use itermore::IterMore;

pub fn run() {
    let input = fs::read_to_string("data/day01a.dat").unwrap();
    let data = parse(&input);
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

fn parse(input: &str) -> Vec<i32> {
    let data = input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line|
            line.parse::<i32>()
                .expect("invalid input data!")
        )
        .collect();

    return data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
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
        "#;
        let data = parse(&input);
        assert_eq!(sonar(&data), 7);
        assert_eq!(three(&data), 5);
    }
}
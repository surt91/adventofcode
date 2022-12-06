use itertools::Itertools;

use aoc2021::data_str;

pub fn run() -> (usize, usize) {

    let input = data_str!("day06");
    let position = end_of_marker(input);

    (
        position,
        0
    )
}

fn end_of_marker(datastream: &str) -> usize {
    // prefix with ' ' to
    let chars: Vec<_> = datastream.chars().collect();
    let index = chars.windows(4)
        .take_while(|window| !window.iter().all_unique())
        .count();
    if index == 0 {
        1
    } else {
        index + 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(end_of_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(end_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(end_of_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(end_of_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(end_of_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(end_of_marker("abcdefg"), 1);
    }
}
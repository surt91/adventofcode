use itertools::Itertools;

use aoc2021::data_str;

pub fn run() -> (usize, usize) {

    let input = data_str!("day06");
    let packet_start = start_of_packet_marker(input);
    let message_start = start_of_message_marker(input);

    (
        packet_start,
        message_start
    )
}

fn start_of_packet_marker(datastream: &str) -> usize {
    find_marker(datastream, 4)
}

fn start_of_message_marker(datastream: &str) -> usize {
    find_marker(datastream, 14)
}

fn find_marker(datastream: &str, length: usize) -> usize {
    let chars: Vec<_> = datastream.chars().collect();
    let index = chars.windows(length)
        .take_while(|window| !window.iter().all_unique())
        .count();
    if index == 0 {
        1
    } else {
        index + length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(start_of_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(start_of_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(start_of_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(start_of_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(start_of_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(start_of_packet_marker("abcdefg"), 1);

        assert_eq!(start_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(start_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(start_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(start_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(start_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
        assert_eq!(start_of_message_marker("abcdefg"), 1);
    }
}
use scan_fmt::scan_fmt;

use aoc2021::data_str;
use aoc2021::utils::split_lines;

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range {start, end}
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.start
        || self.start <= other.end && self.end >= other.end
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day04");
    let data = parse(input);

    (
        count_full_overlap(&data),
        count_partial_overlap(&data)
    )
}

fn count_full_overlap(ranges: &[(Range, Range)]) -> usize {
    ranges.iter()
        .filter(|(range_elf1, range_elf2)|
            range_elf1.contains(range_elf2) || range_elf2.contains(range_elf1)
        )
        .count()
}

fn count_partial_overlap(ranges: &[(Range, Range)]) -> usize {
    ranges.iter()
        .filter(|(range_elf1, range_elf2)|
            range_elf1.overlaps(range_elf2) || range_elf2.contains(range_elf1)
        )
        .count()
}

fn parse(input: &str) -> Vec<(Range, Range)> {
    split_lines(input).iter()
        .flat_map(|line| {
            scan_fmt!(
                line.trim(),
                "{}-{},{}-{}",
                usize, usize, usize, usize
            )
        })
        .map(|(elf1_start, elf1_end, elf2_start, elf2_end)|
            (Range::new(elf1_start, elf1_end), Range::new(elf2_start, elf2_end))
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ";

        let data = parse(input);

        assert_eq!(count_full_overlap(&data), 2);
        assert_eq!(count_partial_overlap(&data), 4);
    }
}
use aoc2021::data_str;
use aoc2021::utils::AdventError;

pub fn run() -> (u64, u64) {

    let input = data_str!("day06");
    let records = parse(input).expect("invalid input");
    let records_without_space = parse(&input.replace(' ', "")).expect("invalid input");

    (
        num_winning_times(&records).into_iter().product(),
        num_winning_times(&records_without_space).into_iter().product(),
    )
}

fn parse(s: &str) -> Result<Vec<(u64, u64)>, AdventError> {
    let mut lines = s.trim().split('\n');
    let times: Vec<u64> = lines.next()
        .ok_or(AdventError::NotEnoughElements)?
        .trim()
        .strip_prefix("Time:")
        .ok_or(AdventError::UnexpectedElement { found: s.to_string(), expected: &["Time:"] })?
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let distances: Vec<u64> = lines.next()
        .ok_or(AdventError::NotEnoughElements)?
        .trim()
        .strip_prefix("Distance:")
        .ok_or(AdventError::UnexpectedElement { found: s.to_string(), expected: &["Distance:"] })?
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(
        times.into_iter()
            .zip(distances)
            .collect()
    )
}

fn num_winning_times(records: &[(u64, u64)]) -> Vec<u64> {
    records.iter().map(|(time, distance)| {
        (0..*time).map(|i| i * (time - i))
            .filter(|i| i > distance)
            .count() as u64
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            Time:      7  15   30
            Distance:  9  40  200
        ";

        let records = parse(input).expect("invalid input");
        let records_without_space = parse(&input.replace(' ', ""))
            .expect("invalid input");

        assert_eq!(num_winning_times(&records).into_iter().product::<u64>(), 288);
        assert_eq!(num_winning_times(&records_without_space).into_iter().product::<u64>(), 71503);
    }
}
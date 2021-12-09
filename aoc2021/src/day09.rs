use crate::utils::{read_lines};


// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

// 5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

pub fn run() -> (usize, usize) {
    let lines = read_lines("data/day09a.dat");
    let parsed = parse(&lines);

    (
        risk(&parsed),
        0,
    )
}

fn risk(map: &[Vec<u8>]) -> usize {
    let width = map[0].len();
    let height = map.len();
    assert!(map.iter().all(|l| l.len() == width));

    let mut risk_rating = 0;
    for y in 0..height {
        for x in 0..width {
            let depth = map[y][x];
            let neighbors = vec![
                if y == 0 {None} else {Some(&map[y-1][x])},
                map.get(y+1).and_then(|line| line.get(x)),
                if x == 0 {None} else {Some(&map[y][x-1])},
                map[y].get(x+1),
            ];

            if neighbors.iter()
                .filter(|n| n.is_some())
                .all(|n| depth < *n.unwrap())
            {
                risk_rating += 1 + depth as usize
            }
        }
    }

    risk_rating
}

fn parse(strings: &[String]) -> Vec<Vec<u8>> {
    strings.iter().map(|line|
        line.chars()
            .map(|c| c.to_digit(10)
                .unwrap() as u8
            ).collect()
    ).collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example() {
        let input = r"
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ";

        let lines = parse(&split_lines(input));

        assert_eq!(risk(&lines), 15);
    }
}
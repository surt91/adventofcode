use crate::utils::{read_lines, AdventError};

type Line = ([String; 10], [String; 4]);

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

const NUM2SEGMENTS: [usize; 10] = [6, 2, 5, 5, 4, 5, 5, 3, 7, 6];


pub fn run() -> (usize, usize) {
    let lines = read_lines("data/day08a.dat");
    let parsed = parse(&lines).expect("invalid input");

    (
        num_1478(&parsed),
        0,
    )
}

fn num_1478(results: &[Line]) -> usize {
    results.iter()
        .map(|ele| ele.1.clone())
        .flatten()
        .filter(|ele| matches!(ele.len(), 2|4|3|7))
        .count()
}

fn parse(input: &[String]) -> Result<Vec<Line>, AdventError> {
    input.iter()
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(input: &str) -> Result<Line, AdventError> {
    let mut it = input.split('|');
    let first = it.next().ok_or(AdventError::NotEnoughElements)?.trim();
    let second = it.next().ok_or(AdventError::NotEnoughElements)?.trim();

    let first: [String; 10] = first.split(' ').map(|s| s.to_string()).collect::<Vec<String>>().try_into().map_err(|_| AdventError::WrongNumberOfElements)?;
    let second: [String; 4] = second.split(' ').map(|s| s.to_string()).collect::<Vec<String>>().try_into().map_err(|_| AdventError::WrongNumberOfElements)?;

    Ok((first, second))
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn example() {
        let input = r"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ";

        let lines = parse(&split_lines(input)).expect("invalid input");

        assert_eq!(num_1478(&lines), 26);
    }
}
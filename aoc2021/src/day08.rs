use std::collections::{HashMap, HashSet};

use crate::{utils::{AdventError, split_lines}, data_str};

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

pub fn run() -> (usize, usize) {
    let input = data_str!("day08a");
    let lines = split_lines(input);
    let parsed = parse(&lines).expect("invalid input");

    (
        num_1478(&parsed),
        sum(&parsed),
    )
}

fn num_1478(results: &[Line]) -> usize {
    results.iter()
        .flat_map(|ele| ele.1.clone())
        .filter(|ele| matches!(ele.len(), 2|4|3|7))
        .count()
}

fn sum(input: &[Line]) -> usize {
    input.iter().map(|line| {
        let table = decode_line(line);
        line.1.iter().rev().enumerate().map(|(n, digit)| {
            let d = segments2digit(&table, digit);
            d as usize * 10usize.pow(n as u32)
        }).sum::<usize>()
    }).sum()
}

fn segments2digit(translation: &HashMap<char, char>, segments: &str) -> u8 {
    let mut translated: Vec<char> = segments.chars().map(|c| translation[&c]).collect();
    translated.sort_unstable();
    let translated: String = translated.iter().collect();

    match translated.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("invalid input")
    }
}

// this is what I call brute force...
fn decode_line(line: &Line) -> HashMap<char, char> {
    let numbers = &line.0;
    let mut translation: HashMap<char, char> = HashMap::new();

    let one: HashSet<char> = numbers.iter().find(|x| x.len() == 2).unwrap().chars().collect();
    let four: HashSet<char> = numbers.iter().find(|x| x.len() == 4).unwrap().chars().collect();
    let seven: HashSet<char> = numbers.iter().find(|x| x.len() == 3).unwrap().chars().collect();
    let eight: HashSet<char> = numbers.iter().find(|x| x.len() == 7).unwrap().chars().collect();

    let mut zero_or_nine_or_six = numbers.iter().filter(|x| x.len() == 6);

    let c096a: HashSet<char> = zero_or_nine_or_six.next().unwrap().chars().collect();
    let c096b: HashSet<char> = zero_or_nine_or_six.next().unwrap().chars().collect();
    let c096c: HashSet<char> = zero_or_nine_or_six.next().unwrap().chars().collect();

    let (six, c09a, c09b) = if one.difference(&c096a).count() == 1 {
        (c096a, c096b, c096c)
    } else if one.difference(&c096b).count() == 1 {
        (c096b, c096a, c096c)
    } else if one.difference(&c096c).count() == 1 {
        (c096c, c096b, c096a)
    } else {
        panic!("invalid input");
    };

    let c = one.difference(&six).next().unwrap();
    translation.insert(*c, 'c');

    // the difference between 1 and 7 -> a
    let a = seven.difference(&one).next().unwrap();
    translation.insert(*a, 'a');

    let (zero, nine) = if four.difference(&c09a).count() == 0 {
        (c09b, c09a)
    } else {
        (c09a, c09b)
    };

    let d = eight.difference(&zero).next().unwrap();
    translation.insert(*d, 'd');
    let e = eight.difference(&nine).next().unwrap();
    translation.insert(*e, 'e');

    let mut four_a = four;
    four_a.insert(*a);
    let g = nine.difference(&four_a).next().unwrap();
    translation.insert(*g, 'g');

    let mut seven_ge = seven.clone();
    seven_ge.insert(*g);
    seven_ge.insert(*e);

    let b = zero.difference(&seven_ge).next().unwrap();
    translation.insert(*b, 'b');

    let f = one.intersection(&six).next().unwrap();
    translation.insert(*f, 'f');

    translation
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
        assert_eq!(sum(&lines), 61229);
    }
}
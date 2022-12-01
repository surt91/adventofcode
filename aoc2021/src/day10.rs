use std::collections::HashSet;

use crate::{data_str, utils::split_lines};

pub fn run() -> (usize, usize) {
    let input = data_str!("day10a");
    let lines = split_lines(input);

    (
        score(&lines),
        middle_score(&lines)
    )
}

fn score(lines: &[String]) -> usize {
    let results = parse(lines);

    results.iter().map(|r| {
        match r {
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            Ok(_) => 0,
            Err(_) => panic!("invalid input")
        }
    }).sum()
}

fn middle_score(lines: &[String]) -> usize {
    let results = parse(lines);

    let mut list = results.iter()
        .flatten()
        .map(|r| {
            completion_score(r)
        }).collect::<Vec<usize>>();

    list.sort_unstable();
    list[list.len() / 2]
}

fn completion_score(completion: &str) -> usize {
    let mut score = 0;
    for c in completion.chars() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("invalid input")
        }
    }

    score
}

fn parse_line(line: &str) -> Result<String, char> {
    let open: HashSet<char> = vec!['(', '[', '{', '<'].into_iter().collect();
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if open.contains(&c) {
            stack.push(c)
        } else {
            let result = match stack.pop() {
                Some('(') => if c == ')' {None} else {Some(c)}
                Some('[') => if c == ']' {None} else {Some(c)}
                Some('{') => if c == '}' {None} else {Some(c)}
                Some('<') => if c == '>' {None} else {Some(c)}
                None => Some(c),
                Some(_) => panic!("invalid input")
            };
            if result.is_some() {
                return Err(c)
            }
        }
    }

    // no error, so start completing
    let completion = stack.iter().rev().map(|c| {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("invalid input")
        }
    }).collect();

    Ok(completion)
}

fn parse(input: &[String]) -> Vec<Result<String, char>> {
    input.iter().map(|line| parse_line(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn completion() {
        assert_eq!(completion_score("}}]])})]"), 288957);
        assert_eq!(completion_score(")}>]})"), 5566);
        assert_eq!(completion_score("}}>}>))))"), 1480781);
        assert_eq!(completion_score("]]}}]}]}>"), 995444);
        assert_eq!(completion_score("])}>"), 294);
    }

    #[test]
    fn example() {
        let input = r"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        ";

        let lines = split_lines(input);

        assert_eq!(score(&lines), 26397);
        assert_eq!(middle_score(&lines), 288957);
    }
}
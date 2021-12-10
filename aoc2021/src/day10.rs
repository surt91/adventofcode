use std::collections::HashSet;

use crate::utils::read_lines;

pub fn run() -> (usize, usize) {
    let lines = read_lines("data/day10a.dat");

    (
        score(&lines),
        0
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
            _ => 0
        }
    }).sum()
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
                Some(_) => unreachable!(),
                None => Some(c)
            };
            if result.is_some() {
                return Err(c)
            }
        }
    }

    Ok(line.to_string())
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
    }
}
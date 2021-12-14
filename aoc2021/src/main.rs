use aoc2021::*;

use std::{env, fmt::Display};

use colored::Colorize;

fn main() {
    let mut args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        // without argument, run all
        args = (0..=25).map(|n| format!("{}", n)).collect();
    }

    fn show<T, U>(n: usize, (a, b): (T, U))
        where T: Display, U: Display
    {
        let title = format!("Day {}", n).yellow();
        println!("{}", title);
        println!("{}", a);
        println!("{}", b);
        println!();
    }

    for arg in &args[1..] {
        match arg.parse::<usize>() {
            Ok(n @ 1) => show(n, day01::run()),
            Ok(n @ 2) => show(n, day02::run()),
            Ok(n @ 3) => show(n, day03::run()),
            Ok(n @ 4) => show(n, day04::run()),
            Ok(n @ 5) => show(n, day05::run()),
            Ok(n @ 6) => show(n, day06::run()),
            Ok(n @ 7) => show(n, day07::run()),
            Ok(n @ 8) => show(n, day08::run()),
            Ok(n @ 9) => show(n, day09::run()),
            Ok(n @ 10) => show(n, day10::run()),
            Ok(n @ 11) => show(n, day11::run()),
            Ok(n @ 12) => show(n, day12::run()),
            Ok(n @ 13) => show(n, day13::run()),
            Ok(n @ 14) => show(n, day14::run()),
            Ok(n) => println!("{}\nIs not solved yet!\n", format!("Day {}", n).yellow()),
            Err(e) => println!("Invalid Argument: {}.", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    day_tests!{
        day01: (1564, 1611),
        day02: (1383564, 1488311643),
        day03: (738234, 3969126),
        day04: (34506, 7686),
        day05: (5092, 20484),
        day06: (360268, 1632146183902),
        day07: (349769, 99540554),
        day08: (301, 908067),
        day09: (514, 1103130),
        day10: (193275, 2429644557),
        day11: (1603, 222),
        day12: (4411, 136767),
        day13: (704, "HGAJBEHC"),
        day14: (0, 0),
    }
}

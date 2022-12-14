use aoc2022::*;

use std::{env, fmt::Display};

use colored::Colorize;

fn main() {
    let mut args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        // without argument, run all
        args = (0..=25).map(|n| format!("{n}")).collect();
    }

    fn show<T, U>(n: usize, (a, b): (T, U))
        where T: Display, U: Display
    {
        let title = format!("Day {n}").yellow();
        println!("{title}");
        println!("{a}");
        println!("{b}");
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
            Ok(n) => println!("{}\nIs not solved yet!\n", format!("Day {n}").yellow()),
            Err(e) => println!("Invalid Argument: {e}."),
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc2021::test;
    use crate::*;

    test!{day01: (67027, 197291)}
    test!{day02: (13052, 13693)}
    test!{day03: (7793, 2499)}
    test!{day04: (550, 931)}
    test!{day05: ("ZWHVFWQWW", "HZFZCCWWV")}
    test!{day06: (1109, 3965)}
    test!{day07: (919137, 2877389)}
    test!{day08: (1840, 405769)}
    test!{day09: (6494, 2691)}
    test!{day10: (14560, "EKRHEPUZ")}
    test!{day11: (117624, 16792940265)}
    test!{day12: (420, 414)}
    test!{day13: (6623, 23049)}
    test!{day14: (888, 0)}
}

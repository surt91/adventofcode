use aoc2023::*;

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
            Ok(n) => println!("{}\nIs not solved yet!\n", format!("Day {n}").yellow()),
            Err(e) => println!("Invalid Argument: {e}."),
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc2021::test;
    use crate::*;

    test!{day01: (55621, 53592)}
    test!{day02: (2149, 71274)}
    test!{day03: (527369, 73074886)}
    test!{day04: (24175, 18846301)}
    test!{#day05: (278755257, 26829166)}
    test!{day06: (2344708, 30125202)}
    test!{day07: (246424613, 248256639)}
    test!{day08: (19667, 19185263738117)}
    test!{day09: (1757008019, 995)}
    test!{day10: (6773, 493)}
}

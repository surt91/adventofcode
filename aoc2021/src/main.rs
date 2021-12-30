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
            Ok(n @ 15) => show(n, day15::run()),
            Ok(n @ 16) => show(n, day16::run()),
            Ok(n @ 17) => show(n, day17::run()),
            Ok(n @ 18) => show(n, day18::run()),
            Ok(n @ 19) => show(n, day19::run()),
            Ok(n @ 20) => show(n, day20::run()),
            Ok(n @ 21) => show(n, day21::run()),
            Ok(n @ 22) => show(n, day22::run()),
            Ok(n @ 23) => show(n, day23::run()),
            Ok(n @ 24) => show(n, day24::run()),
            Ok(n) => println!("{}\nIs not solved yet!\n", format!("Day {}", n).yellow()),
            Err(e) => println!("Invalid Argument: {}.", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    test!{day01: (1564, 1611)}
    test!{day02: (1383564, 1488311643)}
    test!{day03: (738234, 3969126)}
    test!{day04: (34506, 7686)}
    test!{day05: (5092, 20484)}
    test!{day06: (360268, 1632146183902)}
    test!{day07: (349769, 99540554)}
    test!{day08: (301, 908067)}
    test!{day09: (514, 1103130)}
    test!{day10: (193275, 2429644557)}
    test!{day11: (1603, 222)}
    test!{day12: (4411, 136767)}
    test!{day13: (704, "HGAJBEHC")}
    test!{day14: (5656, 12271437788530)}
    test!{day15: (498, 2901)}
    test!{day16: (981, 299227024091)}
    test!{day17: (3916, 2986)}
    test!{day18: (3675, 4650)}
    test!{#day19: (318, 12166)}
    test!{day20: (5583, 19592)}
    test!{day21: (503478, 716241959649754)}
    test!{day22: (658691, 1228699515783640)}
    test!{day23: (14350, 49742)}
    test!{#day24: (98998519596997, 31521119151421)}
}

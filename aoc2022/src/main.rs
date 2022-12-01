use aoc2022::*;

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
            Ok(n) => println!("{}\nIs not solved yet!\n", format!("Day {}", n).yellow()),
            Err(e) => println!("Invalid Argument: {}.", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc2021::test;
    use crate::*;

    test!{day01: (67027, 197291)}
}

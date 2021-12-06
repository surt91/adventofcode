use aoc2021::*;

use std::{env, fmt::Display};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Sepcify for which day the the solution should be printed.")
    }

    fn show<T, U>((a, b): (T, U))
        where T: Display, U: Display
    {
        println!("{}", a);
        println!("{}", b);
    }

    for arg in &args[1..] {
        match arg.parse::<i32>() {
            Ok(1) => show(day01::run()),
            Ok(2) => show(day02::run()),
            Ok(3) => show(day03::run()),
            Ok(4) => show(day04::run()),
            Ok(5) => show(day05::run()),
            Ok(6) => show(day06::run()),
            Err(e) => println!("Invalid Argument: {}.", e),
            _ => println!("Is not solved yet!")
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
    }
}

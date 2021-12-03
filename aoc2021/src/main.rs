use aoc2021::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Sepcify for which day the the solution should be printed.")
    }

    match args[1].parse::<i32>() {
        Ok(1) => day01::run(),
        Ok(2) => day02::run(),
        Ok(3) => day03::run(),
        Err(e) => println!("Invalid Argument: {}.", e),
        _ => println!("Is not solved yet!")
    }
}

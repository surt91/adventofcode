use criterion::{criterion_group, criterion_main, Criterion};

#[macro_export]
macro_rules! day_benches {
    ($($name:ident,)*) => {
        $(
            fn $name(c: &mut Criterion) {
                c.bench_function(stringify!($name), |b| b.iter(aoc2021::$name::run));
            }
        )*
        criterion_group!(benches, $($name,)*);
    }
}

day_benches!{
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    // day24,
    day25,
}

criterion_main!(benches);
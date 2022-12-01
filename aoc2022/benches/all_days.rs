use criterion::{criterion_group, criterion_main, Criterion};

#[macro_export]
macro_rules! day_benches {
    ($($name:ident,)*) => {
        $(
            fn $name(c: &mut Criterion) {
                c.bench_function(stringify!($name), |b| b.iter(aoc2022::$name::run));
            }
        )*
        criterion_group!(benches, $($name,)*);
    }
}

day_benches!{
    day01,
}

criterion_main!(benches);
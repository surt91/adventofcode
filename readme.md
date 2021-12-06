# :christmas_tree: Advent of Code

[![Rust](https://github.com/surt91/adventofcode/actions/workflows/rust.yml/badge.svg)](https://github.com/surt91/adventofcode/actions/workflows/rust.yml)

My solutions for Advent of Code.

## :hammer_and_wrench: Setup

Ensure that you have `cargo` installed (see, e.g., [rustup](https://rustup.rs/)).

Then change into the subdirectory of the corresponding year (e.g. `aoc2021`), compile with

```bash
cargo build --release
```

and run the problems of the specified days

```bash
cargo run --release -- 2 4 6
```

## :test_tube: Tests

Run the tests for a specific day with

```bash
cargo test day02
```

or run all tests with

```bash
cargo test day02
```

## :robot: Benchmarks

Every day can also be benchmarked with

```bash
cargo bench day02
```

or benchmark all days (which will take a while) with

```bash
cargo bench
```

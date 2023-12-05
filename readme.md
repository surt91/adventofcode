# :christmas_tree: Advent of Code

[![Rust](https://github.com/surt91/adventofcode/actions/workflows/rust.yml/badge.svg)](https://github.com/surt91/adventofcode/actions/workflows/rust.yml)

My solutions for Advent of Code.

## :hammer_and_wrench: Setup

Ensure that you have `cargo` installed (see, e.g., [rustup](https://rustup.rs/)).

Run the problems of the specified days of the latest year

```bash
cargo run --release -- 2 4 6
```

or a specific year

```bash
cargo run --release -p aoc2021 -- 23 25
```

or of all days of the latest year

```bash
cargo run --release
```

## :test_tube: Tests

Run the tests for a specific day and yeat with

```bash
cargo test -p aoc2022 day02
```

or run all tests with

```bash
cargo test --all
```

## :racing_car: Benchmarks

Every day can also be benchmarked with

```bash
cargo bench -p aoc2023 day02
```

or benchmark all days (which will take a while) with

```bash
cargo bench --all
```

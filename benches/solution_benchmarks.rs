use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc_2024::day1;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 01, Part 1", |b| {
        b.iter(|| day1::part_one(black_box("inputs/day01.txt")))
    });
    c.bench_function("Day 01, Part 2", |b| {
        b.iter(|| day1::part_two(black_box("inputs/day01.txt")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

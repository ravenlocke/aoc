use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc_2024::{day1, day2};

fn criterion_benchmark(c: &mut Criterion) {
    let contents = include_str!("../inputs/day01.txt");
    c.bench_function("Day 01, Part 1", |b| {
        b.iter(|| day1::part1(black_box(contents)))
    });
    c.bench_function("Day 01, Part 2", |b| {
        b.iter(|| day1::part2(black_box(contents)))
    });
    c.bench_function("Day 02, Part 1", |b| {
        b.iter(|| day2::part1(black_box(contents)))
    });
    c.bench_function("Day 02, Part 2", |b| {
        b.iter(|| day2::part2(black_box(contents)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

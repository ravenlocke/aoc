use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc_2024::{day1, day2, day3, day4, day5, day6};

fn criterion_benchmark(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day01.txt");
    c.bench_function("Day 01, Part 1", |b| {
        b.iter(|| day1::part1(black_box(contents)))
    });
    c.bench_function("Day 01, Part 2", |b| {
        b.iter(|| day1::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day02.txt");
    c.bench_function("Day 02, Part 1", |b| {
        b.iter(|| day2::part1(black_box(contents)))
    });
    c.bench_function("Day 02, Part 2", |b| {
        b.iter(|| day2::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day03.txt");
    c.bench_function("Day 03, Part 1", |b| {
        b.iter(|| day3::part1(black_box(contents)))
    });
    c.bench_function("Day 03, Part 2", |b| {
        b.iter(|| day3::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day04.txt");
    c.bench_function("Day 04, Part 1", |b| {
        b.iter(|| day4::part1(black_box(contents)))
    });
    c.bench_function("Day 04, Part 2", |b| {
        b.iter(|| day4::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day05.txt");
    c.bench_function("Day 05, Part 1", |b| {
        b.iter(|| day5::part1(black_box(contents)))
    });
    c.bench_function("Day 05, Part 2", |b| {
        b.iter(|| day5::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day06.txt");
    c.bench_function("Day 06, Part 1", |b| {
        b.iter(|| day6::part1(black_box(contents)))
    });
    c.bench_function("Day 06, Part 2", |b| {
        b.iter(|| day6::part2(black_box(contents)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

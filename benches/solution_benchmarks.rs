use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc_2024::{day1, day10, day11, day2, day3, day4, day5, day6, day7, day8, day9};

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

    let contents: &str = include_str!("../inputs/day07.txt");
    c.bench_function("Day 07, Part 1", |b| {
        b.iter(|| day7::part1(black_box(contents)))
    });
    c.bench_function("Day 07, Part 2", |b| {
        b.iter(|| day7::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day08.txt");
    c.bench_function("Day 08, Part 1", |b| {
        b.iter(|| day8::part1(black_box(contents)))
    });
    c.bench_function("Day 08, Part 2", |b| {
        b.iter(|| day8::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day09.txt");
    c.bench_function("Day 09, Part 1", |b| {
        b.iter(|| day9::part1(black_box(contents)))
    });
    c.bench_function("Day 09, Part 2", |b| {
        b.iter(|| day9::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day10.txt");
    c.bench_function("Day 10, Part 1", |b| {
        b.iter(|| day10::part1(black_box(contents)))
    });
    c.bench_function("Day 10, Part 2", |b| {
        b.iter(|| day10::part2(black_box(contents)))
    });

    let contents: &str = include_str!("../inputs/day11.txt");
    c.bench_function("Day 11, Part 1", |b| {
        b.iter(|| day11::part1(black_box(contents)))
    });
    c.bench_function("Day 11, Part 2", |b| {
        b.iter(|| day11::part2(black_box(contents)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

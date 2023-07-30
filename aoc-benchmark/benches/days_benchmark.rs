use aoc_lib::{day1, day2, day3};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn day1(c: &mut Criterion) {
    let data = fs::read_to_string("../data/1.in").unwrap();
    c.bench_function("Day 1, Part 1", |b| b.iter(|| day1::part1(&data)));
    c.bench_function("Day 1, Part 2", |b| b.iter(|| day1::part2(&data)));
}

fn day2(c: &mut Criterion) {
    let data = fs::read_to_string("../data/2.in").unwrap();
    c.bench_function("Day 2, Part 1", |b| b.iter(|| day2::part1(&data)));
    c.bench_function("Day 2, Part 2", |b| b.iter(|| day2::part2(&data)));
}

fn day3(c: &mut Criterion) {
    let data = fs::read_to_string("../data/3.in").unwrap();
    c.bench_function("Day 3, Part 1", |b| b.iter(|| day3::part1(&data)));
    c.bench_function("Day 3, Part 2", |b| b.iter(|| day3::part2(&data)));
}

criterion_group!(benches, day1, day2);
criterion_main!(benches);

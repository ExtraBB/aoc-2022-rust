use aoc_lib::day1;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let data = fs::read_to_string("../data/1.in").unwrap();
    c.bench_function("Part 1", |b| b.iter(|| day1::part1(&data)));
    c.bench_function("Part 2", |b| b.iter(|| day1::part2(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

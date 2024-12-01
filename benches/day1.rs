use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/2024/day1.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(|| aoc2024::day1::part1(INPUT)));
    c.bench_function("part 2", |b| b.iter(|| aoc2024::day1::part2(INPUT)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

aoc_lib! { year = 2024 }

#[macro_export]
macro_rules! benchmark {
    ($($day:ident),*) => {
        $(
            use criterion::{criterion_group, criterion_main, Criterion};
            const INPUT: &str = include_str!(concat!("../input/2024/", stringify!($day), ".txt"));
            pub fn criterion_benchmark(c: &mut Criterion) {
                c.bench_function(concat!(stringify!($day), " part 1"), |b| b.iter(|| aoc2024::$day::part1(INPUT)));
                c.bench_function(concat!(stringify!($day), " part 2"), |b| b.iter(|| aoc2024::$day::part2(INPUT)));
            }
            criterion_group!(benches, criterion_benchmark);
            criterion_main!(benches);
        )*
    };
}

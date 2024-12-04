use advent_of_code_2024::day1::{part1, part2};
use advent_of_code_2024::day2::{part1 as day2_part1, part2 as day2_part2};
use advent_of_code_2024::day3::{part1 as day3_part1, part2 as day3_part2};
use advent_of_code_2024::day4::{part1 as day4_part1, part2 as day4_part2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/2024/day1.txt");
const DAY_2_INPUT: &str = include_str!("../input/2024/day2.txt");
const DAY_3_INPUT: &str = include_str!("../input/2024/day3.txt");

pub fn day1(c: &mut Criterion) {
    c.bench_function("day1 part1", |b| b.iter(|| part1(black_box(INPUT))));
    c.bench_function("day1 part2", |b| b.iter(|| part2(black_box(INPUT))));
}

pub fn day2(c: &mut Criterion) {
    c.bench_function("day2 part1", |b| b.iter(|| day2_part1(black_box(DAY_2_INPUT))));
    c.bench_function("day2 part2", |b| b.iter(|| day2_part2(black_box(DAY_2_INPUT))));
}

pub fn day3(c: &mut Criterion) {
    c.bench_function("day3 part1", |b| b.iter(|| day3_part1(black_box(DAY_3_INPUT))));
    c.bench_function("day3 part2", |b| b.iter(|| day3_part2(black_box(DAY_3_INPUT))));
}

pub fn day4(c: &mut Criterion) {
    c.bench_function("day4 part1", |b| b.iter(|| day4_part1(black_box(DAY_4_INPUT))));
    c.bench_function("day4 part2", |b| b.iter(|| day4_part2(black_box(DAY_4_INPUT))));
}

criterion_group!(benches, day1, day2, day3);

criterion_main!(benches);
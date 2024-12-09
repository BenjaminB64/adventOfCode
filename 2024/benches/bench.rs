use advent_of_code_2024::day1::{part1, part2};
use advent_of_code_2024::day2::{part1 as day2_part1, part2 as day2_part2};
use advent_of_code_2024::day3::{part1 as day3_part1, part2 as day3_part2};
use advent_of_code_2024::day4::{part1 as day4_part1, part2 as day4_part2};
use advent_of_code_2024::day5::{part1 as day5_part1, part2 as day5_part2};
use advent_of_code_2024::day6::{part1 as day6_part1, part2 as day6_part2};
use advent_of_code_2024::day7::{part1 as day7_part1, part2 as day7_part2};
use advent_of_code_2024::day8::{part1 as day8_part1, part2 as day8_part2};
use advent_of_code_2024::day9::{part1 as day9_part1, part2 as day9_part2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/2024/day1.txt");
const DAY_2_INPUT: &str = include_str!("../input/2024/day2.txt");
const DAY_3_INPUT: &str = include_str!("../input/2024/day3.txt");
const DAY_4_INPUT: &str = include_str!("../input/2024/day4.txt");
const DAY_5_INPUT: &str = include_str!("../input/2024/day5.txt");
const DAY_6_INPUT: &str = include_str!("../input/2024/day6.txt");
const DAY_7_INPUT: &str = include_str!("../input/2024/day7.txt");
const DAY_8_INPUT: &str = include_str!("../input/2024/day8.txt");
const DAY_9_INPUT: &str = include_str!("../input/2024/day9.txt");

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
    //c.bench_function("day4 part2", |b| b.iter(|| day4_part2(black_box(DAY_4_INPUT))));
}

pub fn day5(c: &mut Criterion) {
    c.bench_function("day5 part1", |b| b.iter(|| day5_part1(black_box(DAY_5_INPUT))));
    c.bench_function("day5 part2", |b| b.iter(|| day5_part2(black_box(DAY_5_INPUT))));
}

pub fn day6(c: &mut Criterion) {
    c.bench_function("day6 part1", |b| b.iter(|| day6_part1(black_box(DAY_6_INPUT))));
    //c.bench_function("day6 part2", |b| b.iter(|| day6_part2(black_box(DAY_6_INPUT))));
}

pub fn day7(c: &mut Criterion) {
    c.bench_function("day7 part1", |b| b.iter(|| day7_part1(black_box(DAY_7_INPUT))));
    c.bench_function("day7 part2", |b| b.iter(|| day7_part2(black_box(DAY_7_INPUT))));
}

pub fn day8(c: &mut Criterion) {
    c.bench_function("day8 part1", |b| b.iter(|| day8_part1(black_box(DAY_8_INPUT))));
    c.bench_function("day8 part2", |b| b.iter(|| day8_part2(black_box(DAY_8_INPUT))));
}

pub fn day9(c: &mut Criterion) {
    c.bench_function("day9 part1", |b| b.iter(|| part1(black_box(INPUT))));
    c.bench_function("day9 part2", |b| b.iter(|| part2(black_box(INPUT))));
}


criterion_group!(benches, day1, day2, day3, day4, day5, day6, day7, day8, day9);

criterion_main!(benches);
mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day6::part1::parse_input_day6_part1;
use crate::day6::part2::parse_input_day6_part2;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    parse_input_day6_part1(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    parse_input_day6_part2(input)
}


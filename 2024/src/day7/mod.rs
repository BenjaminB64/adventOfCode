mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day7::part1::{find_equation, parse_input_day7};
use crate::day7::part2::find_equation_part2;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    find_equation(&parse_input_day7(input))
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    find_equation_part2(&parse_input_day7(input))
}


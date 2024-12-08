mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day7::part1::{find_equation, parse_input_day7};

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let equations = parse_input_day7(input);
    
    find_equation(&equations)
    
}

#[aoc(day7, part2)]
pub fn part2(_input: &str) -> u32 {
    0
}


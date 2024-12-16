mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day12::part1::{calc_fence_price, parse_input_day12};
use crate::day12::part2::{calc_fence_price_part2};

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u32 {
    calc_fence_price(&mut parse_input_day12(input))
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u32 {
    calc_fence_price_part2(&mut parse_input_day12(input))
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r"AAABBBD
CCCDDDD";
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1930);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 1206);
    }
}
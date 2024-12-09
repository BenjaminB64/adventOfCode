mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day9::part1::{calc_checksum, parse_input_day9};
use crate::day9::part2::calc_checksum_part2;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    calc_checksum(parse_input_day9(input))
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    calc_checksum_part2(parse_input_day9(input))
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r"2333133121414131402";
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2858);
    }
}
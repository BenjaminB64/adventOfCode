mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day11::part1::{calc_stones, parse_input_day11};

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    calc_stones(&mut parse_input_day11(input), 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    calc_stones(&mut parse_input_day11(input), 75)
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r"125 17
";
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 55312);
    }
}
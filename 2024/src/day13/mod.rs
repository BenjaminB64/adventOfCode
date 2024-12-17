mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day13::part1::{calc_needed_tokens, parse_input};
use crate::day13::part2::calc_needed_tokens_part2;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u32 {
    calc_needed_tokens(parse_input(input))
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    calc_needed_tokens_part2(parse_input(input))
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 480);
    }
    #[test]
    fn part2_example() {
        assert_eq!(0, 0);
    }
}
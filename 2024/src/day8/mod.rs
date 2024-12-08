mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day8::part1::{parse_input_day8, run_simulation};
use crate::day8::part2::run_simulation_part2;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
    run_simulation(&mut parse_input_day8(input))
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
    run_simulation_part2(&mut parse_input_day8(input))
}



#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 34);
    }
}
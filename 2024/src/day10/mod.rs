mod models;
mod part2;
mod part1;

use aoc_runner_derive::{aoc};
use crate::day10::part1::{parse_input_day9, find_trailheads};

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u32 {
    find_trailheads(&parse_input_day9(input))
}

#[aoc(day10, part2)]
pub fn part2(_input: &str) -> u64 {
    0
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1("...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
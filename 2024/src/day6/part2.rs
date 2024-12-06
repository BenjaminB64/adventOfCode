use crate::day6::models::Guardian;
use crate::day6::models::Direction;
use crate::day6::part1::parse_file;

pub fn parse_input_day6_part2(input: &str) -> u32 {
    let (coord_obs, coord_guardian, size_x, size_y) = parse_file(input.as_bytes());
    let guardian = Guardian {
        coord: coord_guardian,
        direction: Direction::Up,
    };
    
    guardian.look_for_loops(&coord_obs, size_x, size_y)
}
use std::collections::HashMap;
use crate::day6::models::{Coord, Direction};

pub fn parse_file(file: &[u8]) -> (Vec<Coord>, Coord, u32, u32) {
    let mut coord_obs: Vec<Coord> = Vec::new();
    let mut coord_guardian: Coord = (0, 0);
    let input_bytes: Vec<&[u8]> = file.split(|x| *x == b'\n').collect();
    for i in 0..input_bytes.len() {
        for j in 0..input_bytes[i].len() {
            if input_bytes[i][j] == b'#' {
                coord_obs.push((j as u32, i as u32));
            }
            if input_bytes[i][j] == b'^' {
                coord_guardian = (j as u32, i as u32);
            }
        }
    }
    (coord_obs, coord_guardian, input_bytes[0].len() as u32, input_bytes.len() as u32)
    
}

pub fn parse_input_day6_part1(input: &str) -> u32 {
    let (coord_obs, mut coord_guardian, size_x, size_y) = parse_file(input.as_bytes());
    let mut go_through_coords: HashMap<(u32, u32), _> = HashMap::new();
    
    go_through_coords.insert(coord_guardian, true);
    let mut new_coord_guardian ;
    let mut direction = Direction::Up;
    let limit_x = size_x - 1;
    let limit_y = size_y - 1;
    
    loop {
        if coord_guardian.0 == 0 || coord_guardian.0 == limit_x || coord_guardian.1 == 0 || coord_guardian.1 == limit_y {
            break;
        }

        new_coord_guardian = try_to_move(&coord_guardian, &direction, &coord_obs);
        if new_coord_guardian == coord_guardian {
            direction = direction.turn_right();
            continue;
        }
        go_through_coords.insert(coord_guardian, true);
        coord_guardian = new_coord_guardian;
    }
    go_through_coords.insert(coord_guardian, true);
    go_through_coords.len() as u32
}
fn try_to_move(coord: &(u32, u32), direction: &Direction, coord_obs: &Vec<(u32, u32)>) -> (u32, u32) {
    let mut new_coord = coord.clone();

    match direction {
        Direction::Up => {
            new_coord.1 -= 1;
        }
        Direction::Down => {
            new_coord.1 += 1;
        }
        Direction::Left => {
            new_coord.0 -= 1;
        }
        Direction::Right => {
            new_coord.0 += 1;
        }
    }
    if coord_obs.contains(&new_coord) {
        return *coord;
    }
    new_coord
}
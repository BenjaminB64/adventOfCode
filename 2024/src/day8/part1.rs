use std::collections::HashMap;
use crate::day8::models::{Map, Point};

pub fn parse_input_day8(input: &str) -> Map {
    let mut points: Vec<Vec<Point>> = vec![];
    let mut width = 0;
    let mut height = 0;
    let mut antenna: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Point> = vec![];
        for (x, c) in line.chars().enumerate() {
            let ant = match c {
                '.' => None,
                _ => Some(c),
            };
            row.push(Point{ant, antinodes: vec![] });
            if ant.is_some() {
                if antenna.contains_key(&c) {
                    antenna.get_mut(&c).unwrap().push((x as u32, y as u32));
                } else {
                    antenna.insert(c, vec![(x as u32, y as u32)]);
                }
            }
        }
        points.push(row);
        width = line.len();
        height = y+1;
    }
    
    Map {
        width: width as u32,
        height: height as u32,
        points,
        antennas: antenna
    }
}

pub fn run_simulation(map: &mut Map) -> u32 {
    let mut new_map = map.clone();
    let mut count = 0;
    for a in map.antennas.iter() {
        for i in 0..a.1.len() {
            for j in i+1..a.1.len() {
                let (x1, y1) = a.1[i];
                let (x2, y2) = a.1[j];
                let antinode1 = (2i32 * (x1 as i32) - x2 as i32, 2i32 * (y1 as i32) - y2 as i32);
                let antinode2 = (2i32 * x2 as i32 - x1 as i32, 2i32 * y2 as i32 - y1 as i32);
                if antinode1.0 >= 0 && antinode1.0 < new_map.width as i32 && antinode1.1 >= 0 && antinode1.1 < new_map.height as i32 {
                    new_map.points[antinode1.1 as usize][antinode1.0 as usize].antinodes.push(*a.0);
                }
                if antinode2.0 >= 0 && antinode2.0 < new_map.width as i32 && antinode2.1 >= 0 && antinode2.1 < new_map.height as i32 {
                    new_map.points[antinode2.1 as usize][antinode2.0 as usize].antinodes.push(*a.0);
                }
            }
        }
    }
    for row in &new_map.points {
        for point in row {
            if point.antinodes.len() > 0{
                count += 1;
            }
        }
    }
    count
}
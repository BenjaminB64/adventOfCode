use std::collections::HashSet;
use crate::day10::models::Map;

pub fn parse_input_day9(input: &str) -> Map {
    let mut rows = vec![];
    let mut width = 0;
    let mut i = 0;
    let mut row = vec![];
    for b in input.bytes() {
        i += 1;
        if b == b'\n' {
            if width == 0 {
                width = i-1;
            }
            rows.push(row);
            row = vec![];
        } else {
            row.push(b);
        }
    }
    if row.len() > 0 {
        rows.push(row);
    }
    let height = rows.len() as i32;
    let map = Map{rows, width, height};
    //println!("{}", map.to_string());
    map
}

pub fn find_trailheads(map: &Map) -> u32 {
    let mut total = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == b'0' {
                let mut trailheads: HashSet<(u32, u32)> = HashSet::new();
                find_trailhead(map, x, y, &mut trailheads, b'0');
                total += trailheads.len() as u32;
                //println!("{}", map.to_string(trailheads));
            }
        }
    }
    total
}

fn find_trailhead(map: &Map, x: i32, y: i32, score: &mut HashSet<(u32, u32)>, look_for: u8) {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {
        return;
    }
    let v = map.get(x, y);

    // println!("{} {} {} {} {}", x, y, v as char, map.width, map.height);
    if v == b'9' {
        if v == look_for {
            score.insert((x as u32, y as u32));
            //println!("Found trailhead at {}, {}", x, y);
        }
        return;
    }

    if v == look_for {
        let new_look_for = look_for + 1;
        find_trailhead(map, x + 1, y, score, new_look_for);
        find_trailhead(map, x - 1, y, score, new_look_for);
        find_trailhead(map, x, y + 1, score, new_look_for);
        find_trailhead(map, x, y - 1, score, new_look_for);
    }
}
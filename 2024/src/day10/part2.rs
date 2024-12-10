use std::collections::HashSet;
use crate::day10::models::Map;

pub fn find_trailheads_part2(map: &Map) -> u32 {
    let mut total = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == b'0' {
                let mut score = 0;
                find_trailhead_part2(map, x, y, &mut score, b'0');
                total += score;       
                //println!("{}", map.to_string(trailheads));
            }
        }
    }
    total
}

fn find_trailhead_part2(map: &Map, x: i32, y: i32, score: &mut u32, look_for: u8) {
    if x < 0 || y < 0 || x >= map.width || y >= map.height {
        return;
    }
    let v = map.get(x, y);

    // println!("{} {} {} {} {}", x, y, v as char, map.width, map.height);
    if v == b'9' {
        if v == look_for {
            *score += 1;
            //println!("Found trailhead at {}, {}", x, y);
        }
        return;
    }

    if v == look_for {
        let new_look_for = look_for + 1;
        find_trailhead_part2(map, x + 1, y, score, new_look_for);
        find_trailhead_part2(map, x - 1, y, score, new_look_for);
        find_trailhead_part2(map, x, y + 1, score, new_look_for);
        find_trailhead_part2(map, x, y - 1, score, new_look_for);
    }
}
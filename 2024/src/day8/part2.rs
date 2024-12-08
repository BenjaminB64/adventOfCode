use crate::day8::models::Map;

pub fn run_simulation_part2(map: &mut Map) -> u32 {
    let mut new_map = map.clone();
    let mut count = 0;
    for a in map.antennas.iter() {
        for i in 0..a.1.len() {
            for j in i+1..a.1.len() {
                let (mut x1, mut y1) = a.1[i];
                let (mut x2, mut y2) = a.1[j];
                let mut antinode: (i32, i32);
                loop {
                    antinode = (2 * (x1 as i32) - x2 as i32, 2 * (y1 as i32) - y2 as i32);
                    if antinode.0 < 0 || antinode.0 >= new_map.width as i32 || antinode.1 < 0 || antinode.1 >= new_map.height as i32 {
                        break;
                    }
                    new_map.points[antinode.1 as usize][antinode.0 as usize].antinodes.push(*a.0);
                    x2 = x1;
                    y2 = y1;
                    x1 = antinode.0 as u32;
                    y1 = antinode.1 as u32;
                }
                (x1, y1) = a.1[i];
                (x2, y2) = a.1[j];
                loop {
                    antinode = (2 * x2 as i32 - x1 as i32, 2 * y2 as i32 - y1 as i32);
                    if antinode.0 < 0 || antinode.0 >= new_map.width as i32 || antinode.1 < 0 || antinode.1 >= new_map.height as i32 {
                        break;
                    }
                    new_map.points[antinode.1 as usize][antinode.0 as usize].antinodes.push(*a.0);
                    x1 = x2;
                    y1 = y2;
                    x2 = antinode.0 as u32;
                    y2 = antinode.1 as u32;
                }
            }
        }
    }
    for row in &new_map.points {
        for point in row {
            if point.antinodes.len() > 0{
                if !point.ant.is_some() {
                    count += 1;
                }
            }
        }
    }
    for a in map.antennas.iter() {
        let n = a.1.len() as u32;
        if n > 1 {
            count += n;
        }
    }
    count
}
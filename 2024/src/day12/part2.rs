use std::collections::{HashSet};
use crate::day12::models::{Garden, Region};
pub fn calc_fence_price_part2(garden: &mut Garden) -> u32 {
    let mut regions: Vec<Region> = vec![];
    for y in 0..garden.height {
        for x in 0..garden.width {
            if let Some(plot) = garden.get_plot(x, y) {
                if plot.counted {
                    continue;
                }
                let region = calc_region_part2(garden, x, y);
                regions.push(region);
            }
        }
    }

    let price = regions.iter().map(|r| r.area * r.sides).sum();
    price
}

const DELTAS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn calc_region_part2(garden: &mut Garden, start_x: u32, start_y: u32) -> Region {
    let region_id = garden.get_plot(start_x, start_y).unwrap().id;
    let mut region = Region {
        id: region_id,
        area: 0,
        perimeter: 0,
        sides: 0,
    };

    let mut history: HashSet<(i32, i32)> = HashSet::new();

    let mut queue: Vec<((i32, i32), Vec<bool>)> = vec![((start_x as i32, start_y as i32), vec![true; 4])];
    while let Some((pos, mut count_perimeter_side)) = queue.pop() {
        let neighbors: Vec<(i32, i32)> = DELTAS.iter().map(|(dx, dy)| (pos.0 + dx, pos.1 + dy)).collect();
        history.insert(pos);
        if is_on_perimeter(garden, pos){
            println!("{:?}", pos);
            region.area += 1;
            garden.get_plot(pos.0 as u32, pos.1 as u32).unwrap().counted = true;
        }

        merge_duplicate(pos, &mut queue, &mut count_perimeter_side);

        for (i, neighbor) in neighbors.iter().enumerate() {
            if (is_on_perimeter(garden, *neighbor) && garden.get_plot(neighbor.0 as u32, neighbor.1 as u32).unwrap().id == region_id) || history.contains(neighbor) {
                count_perimeter_side[i] = true;
            } else {
                if count_perimeter_side[i] {
                    region.sides += 1;
                }
                count_perimeter_side[i] = false;
            }
        }

        for (i, neighbor) in neighbors.iter().enumerate() {
            if count_perimeter_side[i] && !history.contains(neighbor) {
                queue.insert(0,(*neighbor, count_perimeter_side.clone()));
            }
        }
    }
    //println!("{:?}", region);
    region
}

fn is_on_perimeter(garden: &Garden, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < garden.width as i32 && pos.1 < garden.height as i32
}

fn merge_duplicate(pos: (i32, i32), queue: &mut Vec<((i32, i32), Vec<bool>)>, count_perimeter_side: &mut Vec<bool>) {
    
    let matches = queue.
        iter().
        enumerate().
        filter(|(_, plot)| plot.0 == pos).
        collect::<Vec<_>>();

    let matching_indices = matches.
        iter().
        map(|(index, _)| *index).
        collect::<HashSet<_>>();

    *count_perimeter_side = matches
        .iter()
        .map(|(_, plot)| &plot.1)
        .fold(count_perimeter_side.clone(), |result_bools, current_bools| {
            result_bools
                .iter()
                .enumerate()
                .map(|(index, result_bool)| *result_bool && current_bools[index])
                .collect::<Vec<_>>()
        });

    *queue = queue
        .iter()
        .enumerate()
        .filter(|(index, _)| !matching_indices.contains(index))
        .map(|(_, plot)| plot.clone())
        .collect::<Vec<_>>();
}
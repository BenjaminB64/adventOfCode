use std::collections::{HashMap, HashSet};
use crate::day12::models::{Edge, Garden, Region};
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
    // Ici, on peut par exemple calculer le prix en fonction du nombre de côtés
    let price = regions.iter().map(|r| r.area * r.sides).sum();
    price
}


fn calc_region_part2(garden: &mut Garden, start_x: u32, start_y: u32) -> Region {
    let region_id = garden.get_plot(start_x, start_y).unwrap().id;
    let mut region = Region {
        id: region_id,
        area: 0,
        perimeter: 0,
        sides: 0,
        edges: HashSet::new(),
    };

    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        if x >= garden.width || y >= garden.height {
            continue;
        }
        let plot = garden.get_plot(x, y);
        if plot.is_none() {
            continue;
        }
        let plot = plot.unwrap();
        if plot.counted || plot.id != region.id {
            continue;
        }

        plot.counted = true;
        region.area += 1;

        let mut xx = x;
        let mut yy = y;
        let mut edge = Edge((xx, yy), (xx, yy));
        loop {
            if garden.get_plot(xx, yy).unwrap().id != region_id {
                region.sides += 1;
                edge.1 = (xx-1, yy);
                break;
            }
            garden.get_plot(xx, yy).unwrap().counted = true;
            if garden.is_right(xx, yy) {
                region.sides += 1;
                edge.1 = (xx, yy);
                break;
            }
            xx += 1;
        }
        
        xx = x;
        yy = y;
        loop {
            if garden.get_plot(xx, yy).unwrap().id != region_id {
                region.sides += 1;
                edge.0 = (xx + 1, yy);
                break;
            }
            garden.get_plot(xx, yy).unwrap().counted = true;
            if garden.is_left(xx, yy) {
                region.sides += 1;
                edge.0 = (xx, yy);
                break;
            }
            xx -= 1;
        }
        edges.insert(edge);
        
        xx = x;
        yy = y;
        edge = Edge((xx, yy), (xx, yy));
        loop {
            if garden.get_plot(xx, yy).unwrap().id != region_id {
                region.sides += 1;
                edge.1 = (xx, yy-1);
                break;
            }
            garden.get_plot(xx, yy).unwrap().counted = true;
            if garden.is_bottom(xx, yy) {
                region.sides += 1;
                edge.1 = (xx, yy);
                break;
            }
            yy += 1;
        }
        
        edges.insert(edge);
        
        xx = x;
        yy = y;
        loop {
            if garden.get_plot(xx, yy).unwrap().id != region_id {
                region.sides += 1;
                edge.0 = (xx, yy + 1);
                break;
            }
            garden.get_plot(xx, yy).unwrap().counted = true;
            if garden.is_top(xx, yy) {
                region.sides += 1;
                edge.0 = (xx, yy);
                break;
            }
            yy -= 1;
        }
        
        edges.insert(edge);
        
    }
    println!("{:?} {:?}", edges, region.sides);
    region
}
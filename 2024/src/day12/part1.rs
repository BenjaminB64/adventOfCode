use crate::day12::models::{Garden, Plot, Region};

pub fn parse_input_day12(input: &str) -> Garden {
    let plots = input.lines().map(|line| {
        line.chars().map(|c| {
            Plot {
                id: ((c as u8) - b'A'),
                counted: false,
            }
        }).collect()
    }).collect::<Vec<Vec<Plot>>>();
    
    // println!("{:?}", plots);
    Garden {
        width: plots[0].len() as u32,
        height: plots.len() as u32,
        plots
    }
}

pub fn calc_fence_price(garden: &mut Garden) -> u32 {
    let mut regions: Vec<Region> = vec![];
    for y in 0..garden.plots.len() {
        for x in 0..garden.plots[y].len() {
            if garden.plots[y][x].counted {
                continue;
            }
            regions.push(calc_region(garden, x, y));
        }
    }
    // println!("{:?}", regions);
    let price = regions.iter().map(|r| r.area * r.perimeter).sum();
    price
}

fn calc_region(garden: &mut Garden, x: usize, y: usize) -> Region {
    let mut region = Region {
        id: garden.plots[y][x].id,
        area: 0,
        perimeter: 0,
        sides: 0,
        edges: Default::default(),
    };
    let mut stack = vec![(x, y)];
    while let Some((x, y)) = stack.pop() {
        if x >= garden.plots[0].len() || y >= garden.plots.len() {
            continue;
        }
        let plot = &mut garden.plots[y][x];
        if plot.counted || plot.id != region.id {
            continue;
        }
        region.area += 1;
        plot.counted = true;
        
        if x == 0 {
            region.perimeter += 1;
        }
        if y == 0 {
            region.perimeter += 1;
        }
        if x == garden.plots[0].len() - 1 {
            region.perimeter += 1;
        }
        if y == garden.plots.len() - 1 {
            region.perimeter += 1;
        }
        if y > 0 && garden.plots[y - 1][x].id != region.id {
            region.perimeter += 1;
        }
        if y < garden.plots.len() - 1 && garden.plots[y + 1][x].id != region.id {
            region.perimeter += 1;
        }
        if x > 0 && garden.plots[y][x - 1].id != region.id {
            region.perimeter += 1;
        }
        if x < garden.plots[0].len() - 1 && garden.plots[y][x + 1].id != region.id {
            region.perimeter += 1;
        }
        if x > 0 {
            stack.push((x - 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if x < garden.plots[0].len() - 1 {
            stack.push((x + 1, y));
        }
        if y < garden.plots.len() - 1 {
            stack.push((x, y + 1));
        }
    }
    
    region
}
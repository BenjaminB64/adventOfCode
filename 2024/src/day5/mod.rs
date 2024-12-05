use std::collections::HashMap;
use aoc_runner_derive::{aoc};

pub fn parse_input_day5_part1(input: &str) -> u32 {
    let mut order: Vec<(u32, u32)> = Vec::new();
    let input_bytes: Vec<&[u8]> = input.split("\n").map(|x| x.as_bytes()).collect();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut separator_line = 0;
    for i in 0..input_bytes.len() {
        if input_bytes[i].len() == 5 {
            let first = (input_bytes[i][0] - b'0') * 10 + (input_bytes[i][1] - b'0');
            let second = (input_bytes[i][3] - b'0') * 10 + (input_bytes[i][4] - b'0');
            order.push((first as u32, second as u32));
        }
        if input_bytes[i].len() == 0 {
            separator_line = i;
            break;
        }
    }
    for i in separator_line + 1..input_bytes.len() {
        let update = input_bytes[i].split(|x| *x == b',').map(|x| ((x[0] - b'0') * 10 + (x[1] - b'0')) as u32).collect();
        updates.push(update);
    }
    let mut correct_ordered = 0;
    for i in 0..updates.len() {
        let mut correct = true;
        let mut points: HashMap<u32, u32> = HashMap::new();

        for j in 0..updates[i].len() {
            for k in 0..order.len() {
                if order[k].0 == updates[i][j] {
                    if updates[i].contains(&order[k].1) {
                        let value = points.get(&order[k].0);
                        if let Some(value) = value {
                            points.insert(order[k].0, *value + 1);
                        } else {
                            points.insert(order[k].0, 1);
                        }
                    }
                }
            }
        }
        let mut points_ordered = points.iter().collect::<Vec<_>>();
        points_ordered.sort_by(|a, b| b.1.cmp(a.1));

        for j in 0..points_ordered.len() {
            if updates[i][j] != *points_ordered[j].0 {
                correct = false;
                break;
            }
        }
        if correct {
            correct_ordered += updates[i][updates[i].len() / 2];
        }
    }
    correct_ordered
}

pub fn parse_input_day5_part2(input: &str) -> u32 {
    let mut order: Vec<(u32, u32)> = Vec::new();
    let input_bytes: Vec<&[u8]> = input.split("\n").map(|x| x.as_bytes()).collect();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut separator_line = 0;
    let mut correct_score = 0;
    for i in 0..input_bytes.len() {
        if input_bytes[i].len() == 5 {
            let first = (input_bytes[i][0] - b'0') * 10 + (input_bytes[i][1] - b'0');
            let second = (input_bytes[i][3] - b'0') * 10 + (input_bytes[i][4] - b'0');
            order.push((first as u32, second as u32));
        }
        if input_bytes[i].len() == 0 {
            separator_line = i;
            break;
        }
    }
    for i in separator_line + 1..input_bytes.len() {
        let update = input_bytes[i].split(|x| *x == b',').map(|x| ((x[0] - b'0') * 10 + (x[1] - b'0')) as u32).collect();
        updates.push(update);
    }

    for i in 0..updates.len() {
        let mut correct = true;
        let mut points: HashMap<u32, u32> = HashMap::new();

        for j in 0..updates[i].len() {
            for k in 0..order.len() {
                if order[k].0 == updates[i][j] {
                    if updates[i].contains(&order[k].1) {
                        let value = points.get(&order[k].0);
                        if let Some(value) = value {
                            points.insert(order[k].0, *value + 1);
                        } else {
                            points.insert(order[k].0, 1);
                        }
                    }
                }
            }
        }
        let mut points_ordered = points.iter().collect::<Vec<_>>();
        points_ordered.sort_by(|a, b| b.1.cmp(a.1));

        for j in 0..points_ordered.len() {
            if updates[i][j] != *points_ordered[j].0 {
                correct = false;
                break;
            }
        }
        if !correct {
            let mut correct_ordered: Vec<u32> = Vec::new();
            for j in 0..points_ordered.len() {
                correct_ordered.push(*points_ordered[j].0);
            }
            for j in correct_ordered.len()..updates[i].len() {
                correct_ordered.push(updates[i][j]);
            }
            correct_score += correct_ordered[correct_ordered.len() / 2];
        }
    }
    correct_score
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    parse_input_day5_part1(input)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    parse_input_day5_part2(input)
}

